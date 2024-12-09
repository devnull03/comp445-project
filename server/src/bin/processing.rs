use csv;
use rand::Rng;
use rusqlite::Connection;
use serde_json::json;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    fmt::{self, Write},
    fs::File,
    hash::Hash,
    path::Path,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Hash, Clone)]
pub struct Record {
    pub id: u32,
    pub title: String,
    pub text: String,
    pub label: u32, // can be either 1 or 0
}

impl Record {
    pub fn to_db_string(&self) -> String {
        format!(
            r#"({}, "{}", "{}", "{}")"#,
            self.id,
            self.title.replace(r#"""#, "”"),
            self.text.replace(r#"""#, "”"),
            self.label
        )
    }
}

impl<'a> From<&tokio_rusqlite::Row<'a>> for Record {
    fn from(value: &tokio_rusqlite::Row<'a>) -> Self {
        Record {
            id: value.get(0).unwrap(),
            title: value.get(1).unwrap(),
            text: value.get(2).unwrap(),
            label: value.get::<_, String>(3).unwrap().parse().unwrap(),
        }
    }
}

pub type InverseIndexDB = HashMap<String, HashSet<u32>>;

#[allow(dead_code)]
fn main() {
    let shingle_size = 3;
    let minhash_length = 20;
    let similarity_threshold = 0.85;
    let file_path =
        Path::new("/home/devnull03/school/COMP455/project/server/src/bin/evaluation.csv");

    let mut minhash_data: HashMap<u32, Vec<u64>> = HashMap::new();
    let mut similarities: HashMap<u32, HashMap<u32, f64>> = HashMap::new();

    // ------------------------------------------------------------------------------------------------------------------------------------------
    //? loading the data

    println!("Loading file in memory....");
    let records = load_data(file_path);
    let combined_strings: Vec<String> = records
        .values()
        .map(|record| record.to_db_string())
        .collect();

    println!("Creating the inverse idex...");
    let inverse_index: HashMap<String, HashSet<u32>> = build_inverted_index(&records);

    // ------------------------------------------------------------------------------------------------------------------------------------------
    //? prepairing the data

    println!("Creating shingles..");
    let shingled_dataset = create_shingles(&combined_strings, shingle_size);

    println!("Generating Hash functions..");
    let hash_funcs = generate_hash_funcs(minhash_length);

    // ------------------------------------------------------------------------------------------------------------------------------------------
    //? actually processing the data

    println!("Generating minhashes and compairing...");
    for (doc1_id, doc1) in &shingled_dataset {
        let doc1_minhash;
        if let Some(_doc1_minhash) = minhash_data.get(doc1_id).cloned() {
            doc1_minhash = _doc1_minhash;
        } else {
            doc1_minhash = generate_minhash_signature(doc1, &hash_funcs);
            minhash_data.insert(*doc1_id, doc1_minhash.clone());
        }

        let mut doc_similarities: HashMap<u32, f64> = HashMap::new();

        // println!("processing {:?}", doc1_id);

        for (doc2_id, doc2) in &shingled_dataset {
            let doc2_minhash;
            if let Some(_doc2_minhash) = minhash_data.get(doc2_id).cloned() {
                doc2_minhash = _doc2_minhash;
            } else {
                doc2_minhash = generate_minhash_signature(doc2, &hash_funcs);
                minhash_data.insert(*doc2_id, doc2_minhash.clone());
            }

            // let e = jaccard(doc1, doc2);
            let e = minhash_similarity(&doc1_minhash, &doc2_minhash); // gives better similarity ratings?

            if e >= similarity_threshold {
                doc_similarities.insert(*doc2_id, e);
            }
        }

        similarities.insert(*doc1_id, doc_similarities);
    }

    println!("Finished calculating similarities");

    // ------------------------------------------------------------------------------------------------------------------------------------------
    //? create files for data storage

    println!("Generating the csv file");
    let similarities_file_path =
        "/home/devnull03/school/COMP455/project/server/src/bin/similarities.csv";
    create_csv_file(similarities_file_path, &similarities);

    println!("Generating the sqlite database");
    create_sqlite_file(
        "processed.db",
        &combined_strings,
        &inverse_index,
        &similarities,
    );

    // ------------------------------------------------------------------------------------------------------------------------------------------
}

pub fn create_sqlite_file(
    file_name: &str,
    records: &Vec<String>,
    inverse_index: &InverseIndexDB,
    similarities: &HashMap<u32, HashMap<u32, f64>>,
) {
    let records_table_headers = vec!["id", "title", "text", "label"];
    let (records_new_table, records_table_values) =
        build_table_creation_commands("records", &records_table_headers, &records);

    let inverse_index_headers = vec!["string", "entries"];
    let inverse_index_string_values: Vec<String> = inverse_index
        // .values()
        .into_iter()
        .map(|v| {
            format!(
                r#"("{}", "[{}]")"#,
                v.0.replace(r#"""#, "”"),
                format!("{:?}", v.1)
                    .strip_prefix("{")
                    .unwrap()
                    .strip_suffix("}")
                    .unwrap()
                    .to_string()
            )
        })
        .collect();

    let (_inverse_index_new_table, inverse_index_table_values) = build_table_creation_commands(
        "inverse_index",
        &inverse_index_headers,
        &inverse_index_string_values,
    );

    let similarities_headers = vec!["document_id", "similar_documents"];
    let mut similarities_values: Vec<String> = Vec::new();

    for doc in similarities {
        let mut val = "[".to_string();
        let doc_len = doc.1.len();
        for (idx, ele) in doc.1.into_iter().enumerate() {
            write!(
                &mut val,
                "{}",
                json!({
                    "doc_id": ele.0,
                    "similarity": ele.1
                })
                .to_string()
            )
            .unwrap();
            if idx != doc_len {
                write!(&mut val, ",").unwrap();
            }
        }
        write!(&mut val, "]").unwrap();
        similarities_values.push(format!(r#"( {}, '{}' )"#, doc.0, val));
    }

    let (_similarities_new_table, similarities_table_values) =
        build_table_creation_commands("similarities", &similarities_headers, &similarities_values);

    // db stuff -----------------------------------------------------------------------------------------

    let db_connection = Connection::open(file_name).unwrap();

    db_connection.execute(&records_new_table, ()).unwrap();
    db_connection
        .execute(
            "CREATE TABLE IF NOT EXISTS inverse_index ( string TEXT PRIMARY KEY, entries TEXT )",
            (),
        )
        .unwrap();
    db_connection.execute("CREATE TABLE IF NOT EXISTS similarities ( document_id INTEGER PRIMARY KEY, similar_documents TEXT )", ()).unwrap();

    let mut counter = 0;
    for table in vec![
        &records_table_values,
        &similarities_table_values,
        &inverse_index_table_values,
    ] {
        println!("inserting values.. table {}", counter);
        counter += 1;
        for val in table {
            if let Err(e) = db_connection.execute(&val, ()) {
                if let rusqlite::Error::SqliteFailure(ref err, _) = e {
                    if err.code == rusqlite::ErrorCode::ConstraintViolation {
                        println!(
                            "Skipping insertion due to unique constraint violation: {:?}",
                            e
                        );
                        continue;
                    }
                }
                panic!("Failed to execute insertion: {:?}", e);
            }
        }
    }

    // tests --------------------------------------------------------------------------------------------
}

// TODO: get rid of this or minimize
pub fn build_table_creation_commands(
    table_name: &str,
    table_headers: &Vec<&str>,
    values: &Vec<String>,
) -> (String, Vec<String>) {
    let table_headers_len = table_headers.len();

    let mut table_creation_string = format!("CREATE TABLE IF NOT EXISTS {} (", table_name);
    let mut insert_statements: Vec<String> = Vec::new();
    let values_insertion_string = format!("INSERT INTO {} VALUES ", table_name);

    for ele in table_headers.into_iter().enumerate() {
        if ele.1.contains("id") && ele.0 == 0 {
            write!(
                &mut table_creation_string,
                "{} INTEGER PRIMARY KEY NOT NULL, ",
                &ele.1
            )
            .unwrap();
            continue;
        }
        write!(&mut table_creation_string, "{} TEXT", ele.1).unwrap();
        if !ele.0.eq(&(table_headers_len - 1)) {
            write!(&mut table_creation_string, ", ").unwrap();
        };
    }
    write!(&mut table_creation_string, ");").unwrap();

    for val in values {
        insert_statements.push(format!("{} {};", &values_insertion_string, val));
    }

    (table_creation_string, insert_statements)
}

pub fn create_csv_file(
    similarities_file_path: &str,
    similarities: &HashMap<u32, HashMap<u32, f64>>,
) {
    let similarities_file = File::create(similarities_file_path).expect("Failed to create file");
    let mut writer = csv::Writer::from_writer(similarities_file);
    writer
        .write_record(&["document_id", "similar_documents"])
        .expect("Failed to write header");

    for (doc1_id, similar_docs) in similarities {
        writer
            .write_record(&[
                doc1_id.to_string(),
                fmt::format(format_args!("{:?}", similar_docs)),
            ])
            .expect("Failed to write record");
    }

    writer.flush().expect("Failed to flush writer");
}

pub fn load_data(file_path: &Path) -> HashMap<u32, Record> {
    let file = File::open(file_path).expect("Failed to open file");

    let mut csv_reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

    let mut final_data: HashMap<u32, Record> = HashMap::new();
    for result in csv_reader.deserialize() {
        let record: Record = result.expect("Record didn't map correctly");
        final_data.insert(record.id, record);
    }
    println!("total records: {}", final_data.len());
    final_data
}

pub fn tokenize(text: &String) -> HashSet<String> {
    let mut tokens: HashSet<String> = HashSet::new();
    for i in text.split_whitespace() {
        for e in i.split(".") {
            tokens.insert(e.to_lowercase());
        }
    }
    //? would this be better for performance
    //?  text.split_whitespace().map(|x| x.to_lowercase())

    tokens
}

pub fn build_inverted_index(records: &HashMap<u32, Record>) -> InverseIndexDB {
    let mut inverted_index: InverseIndexDB = HashMap::new();

    for record in records {
        let record = record.1;
        let combined_string = vec![record.text.as_str(), record.title.as_str()].join(" ");
        let tokenized_text = tokenize(&combined_string);

        for token in tokenized_text {
            if inverted_index.contains_key(&token) {
                inverted_index.get_mut(&token).unwrap().insert(record.id);
            } else {
                // println!("Created key {:?}", &token);
                inverted_index.insert(token, HashSet::from([record.id]));
            }
        }
    }

    inverted_index
}

pub fn create_shingles(documents: &Vec<String>, k: usize) -> HashMap<u32, HashSet<String>> {
    let mut shingled_dataset: HashMap<u32, HashSet<String>> = HashMap::new();

    for document_id in 0..documents.len() {
        let document = documents.get(document_id).unwrap();
        let mut document_shingles: HashSet<String> = HashSet::new();

        for i in 0..(document.len() - k + 1) {
            // println!("{:?}, {:?}, {:?}", document_id, document.len(), i..(i + k));

            if let Some(shingle) = document.get(i..(i + k)) {
                document_shingles.insert(shingle.to_string());
            }
        }

        shingled_dataset.insert(document_id as u32, document_shingles);
    }

    shingled_dataset
}

fn create_hash_func() -> Box<dyn Fn(&str) -> u64> {
    // Generate random coefficients `a` and `b`
    let mut rng = rand::thread_rng();
    let a: u64 = rng.gen_range(10..10_000);
    let b: u64 = rng.gen_range(10..10_000);
    let large_prime: u64 = 95_633;

    // Return a closure that acts as the hash function
    Box::new(move |s: &str| {
        let sum: u64 = s.chars().map(|c| c as u64).sum();
        (a * sum + b) % large_prime
    })
}

fn generate_hash_funcs(k: usize) -> Vec<Box<dyn Fn(&str) -> u64>> {
    // Generate `k` hash functions
    (0..k).map(|_| create_hash_func()).collect()
}

fn generate_minhash_signature(
    data: &HashSet<String>,
    hash_funcs: &Vec<Box<dyn Fn(&str) -> u64>>,
) -> Vec<u64> {
    let mut minhash_signature: Vec<u64> = Vec::new();

    for hash_func in hash_funcs {
        let mut min_value = u64::MAX;
        for shingle in data {
            let hash_value = hash_func(&shingle);
            min_value = min(min_value, hash_value);
        }
        minhash_signature.push(min_value);
    }

    minhash_signature
}

// Calculate the Jaccard similarity between two sets.
pub fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    let intersection: HashSet<_> = a.intersection(b).collect();
    let union: HashSet<_> = a.union(b).collect();
    intersection.len() as f64 / union.len() as f64
}

// Calculate the similarity between two MinHash signatures.
pub fn minhash_similarity(a: &Vec<u64>, b: &Vec<u64>) -> f64 {
    let matches = a.iter().zip(b).filter(|&(x, y)| x == y).count();
    matches as f64 / a.len() as f64
}

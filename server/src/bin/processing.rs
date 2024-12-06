use csv;
use rand::Rng;
use std::cmp::min;
use std::fmt;
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::Path,
};

#[allow(dead_code)]
fn main() {
    let shingle_size = 3;
    let minhash_length = 20;
    let similarity_threshold = 0.8;
    let file_path =
        Path::new("/home/devnull03/school/COMP455/project/server/src/bin/evaluation.csv");

    // ------------------------------------------------------------------------------------------------------------------------------------------
    // loading the data

    let records = load_data(file_path);
    let combined_strings: Vec<String> = records
        .values()
        .map(|record| format!("{} {}", record.title, record.text))
        .collect();
    // let mut inverse_index = build_inverted_index(&records);

    // ------------------------------------------------------------------------------------------------------------------------------------------
    // prepairing the data

    println!("Creating shingles..");
    let shingled_dataset = create_shingles(&combined_strings, shingle_size);

    println!("Generating Hash functions..");
    let hash_funcs = generate_hash_funcs(minhash_length);

    // ------------------------------------------------------------------------------------------------------------------------------------------
    // creating stuff needed to store the processed data

    let mut similarities: HashMap<u32, HashMap<u32, f64>> = HashMap::new();

    let similarities_file_path =
        "/home/devnull03/school/COMP455/project/server/src/bin/similarities.csv";
    let similarities_file = File::create(similarities_file_path).expect("Failed to create file");
    let mut writer = csv::Writer::from_writer(similarities_file);
    writer
        .write_record(&["Document 1", "Similarities"])
        .expect("Failed to write header");
    
    let mut minhash_data: HashMap<u32, Vec<u64>> = HashMap::new();
    
    // ------------------------------------------------------------------------------------------------------------------------------------------
    // actually processing the data

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

        println!("processing {:?}", doc1_id);

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
        writer
            .write_record(&[
                doc1_id.to_string(),
                fmt::format(format_args!("{:?}", similarities.get(doc1_id).unwrap())),
            ])
            .expect("Failed to write record");
    }

    writer.flush().expect("Failed to flush writer");
    println!("Finished calculating similarities");

    // ------------------------------------------------------------------------------------------------------------------------------------------
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Hash, Clone)]
pub struct Record {
    id: u32,
    title: String,
    text: String,
    label: u32, // can be either 1 or 0
}

pub type InverseIndexDB = HashMap<String, HashSet<u32>>;

pub fn load_data(file_path: &Path) -> HashMap<u32, Record> {
    let file = File::open(file_path).expect("Failed to open file");

    let mut csv_reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

    let mut final_data: HashMap<u32, Record> = HashMap::new();
    for result in csv_reader.deserialize() {
        let record: Record = result.expect("Record didn't map correctly");
        final_data.insert(record.id, record);
    }
    final_data
}

pub fn tokenize(text: &String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    for i in text.split_whitespace() {
        tokens.push(i.to_lowercase());
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
                println!("Created key {:?}", &token);
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

/// Calculate the Jaccard similarity between two sets.
pub fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    let intersection: HashSet<_> = a.intersection(b).collect();
    let union: HashSet<_> = a.union(b).collect();
    intersection.len() as f64 / union.len() as f64
}

/// Calculate the similarity between two MinHash signatures.
pub fn minhash_similarity(a: &Vec<u64>, b: &Vec<u64>) -> f64 {
    let matches = a.iter().zip(b).filter(|&(x, y)| x == y).count();
    matches as f64 / a.len() as f64
}

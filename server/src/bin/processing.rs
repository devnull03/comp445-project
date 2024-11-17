// use uint::{U128, U256};
use csv;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::Path,
};

#[allow(dead_code)]
fn main() {
    // let file_path =
    //     Path::new("/home/devnull03/school/COMP455/project/server/src/bin/evaluation.csv");

    // let records = load_data(file_path);

    // let mut inverse_index = build_inverted_index(&records);

    // for _ in 0..3 {
    //     println!("{:?}", inverse_index.next());
    // }

    let shingles = create_shingles(&vec![
        "alakjfklajF".to_string(),
        "Alakjfklajf".to_string(),
        "alAkjfklajf".to_string(),
        "alakjfklbjf".to_string(),
        "alakjfklajf".to_string(),
        "alakfklajf".to_string(),
    ]);

    for item in shingles.iter().map(hash) {
        println!("{:?}", &item)
    }
}

const SHINGLE_SIZE: u32 = 3;
// const HASH_BASE: u32 = 7;
// const HASH_A: u32 = 173;
// const HASH_B: u32 = 137;
// const HASH_LARGE_PRIME: u32 = 95633;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, Hash)]
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

pub fn create_shingles(tokens: &Vec<String>) -> HashSet<String> {
    let mut shingles = HashSet::new();

    for i in 0..(tokens.len() + 1 - SHINGLE_SIZE as usize) {
        shingles.insert(
            tokens
                .get(i..(i + SHINGLE_SIZE as usize))
                .expect("list index exceded allowed shingle size")
                .join(" "),
        );
    }

    shingles
}

pub fn hash(token: &String) -> u64 {
    // was overflowing
    // let byte_rep: u32 = token
    //     .as_bytes()
    //     .iter()
    //     .enumerate()
    //     .map(|(i, &x)| (x as u32) * (HASH_BASE.pow(i as u32)))
    //     .sum();

    // ((HASH_A * byte_rep + HASH_B) % HASH_LARGE_PRIME) as u64

    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    hasher.finish()
}

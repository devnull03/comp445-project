use csv;
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
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct Record {
    id: u32,
    title: String,
    text: String,
    label: u32, // can be either 1 or 0
}

pub type InverseIndexDB = HashMap<String, HashSet<u32>>;

pub fn load_data(file_path: &Path) -> Vec<Record> {
    let file = File::open(file_path).expect("Failed to open file");

    let mut csv_reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

    let mut final_data: Vec<Record> = Vec::new();
    for result in csv_reader.deserialize() {
        let record: Record = result.expect("Record didn't map correctly");

        // println!("{:?}", &record);
        final_data.push(record);
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

pub fn build_inverted_index(records: &Vec<Record>) -> InverseIndexDB {
    let mut inverted_index: InverseIndexDB  = HashMap::new();

    for record in records {
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

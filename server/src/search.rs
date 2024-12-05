use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::bin::processing::{self, build_inverted_index, load_data, InverseIndexDB, Record};

pub fn build_db(file_path: String) -> (HashMap<u32, Record>, InverseIndexDB) {
    let file_path = Path::new(&file_path);

    let records = load_data(file_path);

    let inverted_index = build_inverted_index(&records);

    (records, inverted_index)
}

pub fn search_db<'a>(
    query: &String,
    inverted_index: &InverseIndexDB,
    records: &'a HashMap<u32, Record>,
) -> Vec<&'a Record> {
    let tokens = processing::tokenize(query);

    let mut last_set: HashSet<u32> = HashSet::new();

    for token in &tokens {
        let e = inverted_index.get(token).unwrap();

        if last_set.is_empty() {
            last_set = e.clone();
        } else {
            last_set = e.intersection(&last_set).cloned().collect();
        }
    }

    let results: Vec<&Record> = last_set
        .into_iter().map(|key| records.get(&key).unwrap()).collect();
    results
}

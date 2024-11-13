use std::path::Path;

use crate::bin::processing::{build_inverted_index, load_data, InverseIndexDB, Record};

pub fn build_db(file_path: String) -> (Vec<Record>, InverseIndexDB) {
    let file_path = Path::new(&file_path);

    let records = load_data(file_path);

    let inverted_index = build_inverted_index(&records);

    (records, inverted_index)
}

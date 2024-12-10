# Project Overview

This project implements a search engine using the MinHashing algorithm to efficiently identify similar documents. The system processes records, generates shingles, computes MinHash signatures, and stores the results in both CSV and SQLite database formats.

## Components

### MinHashing Algorithm

MinHashing is utilized to estimate the similarity between documents by comparing their MinHash signatures. The algorithm involves the following steps:

1. **Shingling**: Each document is broken down into a set of contiguous sequences of tokens (shingles).
2. **Hash Functions**: Multiple hash functions are generated to create MinHash signatures for each document.
3. **Signature Generation**: For each document, the minimum hash value from each hash function is recorded, forming the MinHash signature.
4. **Similarity Calculation**: The similarity between two documents is estimated by the proportion of matching entries in their MinHash signatures.

#### Code Snippet: Generating Hash Functions

```rust
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
```

*The above functions generate a set of hash functions with random coefficients, which are used to compute the MinHash signatures for each document.*

### Search Engine

The search engine leverages the MinHash signatures to quickly identify and retrieve documents that are similar to a query document. It involves:

1. **Inverted Index**: An inverted index is built to map tokens to document IDs, facilitating efficient lookup.
2. **Similarity Threshold**: A predefined threshold determines whether two documents are considered similar.
3. **Result Storage**: Similarities are stored in CSV files and an SQLite database for persistent access and further analysis.

#### Code Snippet: Building the Inverted Index

```rust
pub fn build_inverted_index(records: &HashMap<u32, Record>) -> InverseIndexDB {
    let mut inverted_index: InverseIndexDB = HashMap::new();

    for record in records {
        let record = record.1;
        let combined_string = vec![record.text.as_str(), record.title.as_str()].join(" ");
        let tokenized_text = tokenize(&combined_string);

        for token in tokenized_text {
            inverted_index
                .entry(token)
                .or_insert_with(HashSet::new)
                .insert(record.id);
        }
    }

    inverted_index
}
```

*This function constructs an inverted index by iterating over each record, tokenizing the combined text and title, and mapping each token to the corresponding document IDs.*

### Additional Components

- **Data Loading**: Records are loaded from a CSV file into memory, parsed into structured `Record` objects.
  
  ```rust
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
  ```

- **CSV Generation**: The system generates CSV files containing similarity data for easy inspection and usage.

- **SQLite Database**: An SQLite database is created to store records, the inverted index, and similarity information, enabling robust data management and querying capabilities.

## Usage

1. **Data Preparation**: Ensure that the `evaluation.csv` file is present in the specified directory.
2. **Running the Processor**: Execute the Rust program to process the data, generate shingles, compute MinHash signatures, and calculate similarities.
3. **Output**: The program outputs `similarities.csv` and `processed.db` containing the computed similarities and database records, respectively.

## Dependencies

- **Rust**: The project is written in Rust and requires the Rust toolchain.
- **Crates Used**:
  - `csv`: For reading and writing CSV files.
  - `rand`: For generating random numbers used in hash functions.
  - `rusqlite`: For interacting with the SQLite database.
  - `serde_json`: For handling JSON data serialization.

## Conclusion

This project demonstrates an efficient approach to document similarity detection using MinHashing, providing insights into scalable search engine implementation and data management techniques.

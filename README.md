# Project Specification

The main aims is to develop a user-friendly application or system that allows quickly and easily search through a wide range of items. The main feature will be a powerful search system designed to deliver fast and relevant results, ensuring that users can efficiently explore the diverse catalog.

**Submission Requirements:**

**Requirements (Functional and Non-functional):**

- Implement a flexible search functionality for various attributes, such as titles, categories, and keywords.
- Enable dynamic filtering options based on multiple criteria, ensuring adaptability to different datasets.
- Implement sorting options for search results to enhance user experience.
- Integrate a scalable search engine to handle a growing catalog of items efficiently.
- Performance: Achieve fast and efficient search result retrieval, emphasizing response time.
- Usability: Design an intuitive and user-friendly interface for seamless navigation.
- Scalability: Ensure the application can scale to handle large and diverse datasets.

**Design:** The design section includes a data flow diagram illustrating the flow of data and interactions between various components of the Search and Explore Platform application.

**Implementation:** Provide detailed codes in the appendix. The code is well-commented, organized, and structured for clarity and ease of understanding.

**Conclusion:** Summarize the findings with clear information.

**System or Application :** working system with all the Requirements.

**References:** Include a detail list of all the references with working links to view.

**Appendix:** The appendix should include additional supporting materials such as detailed code snippets, sample input/output data, or any other relevant information that enhances the understanding of the project.

**Data references: Please chose the datasets from the following resources:**

https://openlibrary.org/developers/api

https://www.kaggle.com/datasets

# Project Information and Usage

## Project Overview

This project implements a search engine using the MinHashing algorithm to efficiently identify similar documents. The system processes records, generates shingles, computes MinHash signatures, and stores the results in both CSV and SQLite database formats.

### Notice: Why Rust?

We chose Rust for this project primarily due to its performance advantages over Python. Rust's memory safety guarantees and zero-cost abstractions allow us to write highly efficient and reliable code. Additionally, all team members were comfortable using Rust and contributed effectively to the project.

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

## How to Run

### Step 1: Generate the SQLite Database

First, you need to run the `processing.rs` file to generate the SQLite database. This can be done using the following command:

```sh
cargo run --bin processing
```

This command will process the data and generate the `processed.db` file in the `/server` folder.

### Step 2: Run the Server

After the database has been generated, you can run the server using the following command:

```sh
cargo run --bin server
```

The server will start and listen for incoming requests on `127.0.0.1:3000`.

## API Endpoints

- `GET /test`: A test endpoint to verify the server is running.
- `GET /search?query=<search_text>`: Search for records based on the provided search text.
- `GET /search-results?query_id=<search_id>&page=<page>`: Retrieve paginated search results based on the query ID.

## Environment Variables

- `DB_FILE_PATH`: Path to the SQLite database file. Defaults to `processed.db` if not set.

Make sure to have a `.env` file in the `/server` folder with the necessary environment variables.

## Dependencies

The project uses the following dependencies:

- `tokio`
- `serde`
- `serde_json`
- `axum`
- `tower-http`
- `tower`
- `tracing`
- `tracing-subscriber`
- `uuid`
- `csv`
- `regex`
- `rug`
- `rand`
- `rusqlite`
- `dotenv`
- `tokio-rusqlite`

Refer to the `Cargo.toml` file for the exact versions.

## Frontend Setup Instructions

### Prerequisites
- Node.js (version 14.x or higher)
- npm (version 6.x or higher)

### Installation
1. Clone the repository:
    ```sh
    git clone git@github.com:devnull03/comp445-project.git
    ```
2. Navigate to the project directory:
    ```sh
    cd comp445-project/frontend
    ```
3. Install dependencies:
    ```sh
    npm install
    ```

### Running the Application
1. Start the development server:
    ```sh
    npm start
    ```
2. Open your browser and navigate to `http://localhost:3000`.

## Frontend Routes
### `/`
- **Description**: Home page of the application where users can perform searches.
- **Method**: GET

### `/record/[id]`
- **Description**: Page to view detailed information about a specific document.
- **Method**: GET

### `/api/search`
- **Description**: API endpoint to perform a search query.
- **Method**: POST

### `/api/search-results`
- **Description**: API endpoint to get search results for a specific query.
- **Method**: GET

## Frontend Models
### SearchResultsRes
- **Description**: Represents the response of a search query.
- **Fields**:
  - `search_id`: string
  - `data`: RecordResponse[]
  - `number_of_results`: number
  - `page`: number
  - `total_pages`: number

### RecordResponse
- **Description**: Represents a single record and its similar documents.
- **Fields**:
  - `data`: Record
  - `similar_docs`: SimilarityInfoFull[]

### SimilarityInfoFull
- **Description**: Represents a similar document with similarity information.
- **Fields**:
  - `doc`: Record
  - `similarity`: number

### Record
- **Description**: Represents a document in the application.
- **Fields**:
  - `id`: number
  - `title`: string
  - `text`: string
  - `label`: 1 | 0

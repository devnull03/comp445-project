# Project Overview

This project consists of a data processing module and a server module. The data processing module generates an SQLite database from a CSV file, and the server module provides an API to query the data.

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

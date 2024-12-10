# Server-Python

**Note: This folder contains a reference implementation of the server in Python. It is provided for reference purposes only and is not intended to be run as part of the main project. Additionally, it is not fully implemented yet.**

## Overview

The Python implementation mirrors the functionality of the Rust server, providing similar endpoints and data processing logic. It uses Flask for the web server and SQLite for the database.

## Dependencies

To explore the Python implementation, you will need the following dependencies:

- `Flask`
- `Flask-SQLAlchemy`
- `uuid`
- `dotenv`
- `pandas`
- `numpy`

You can install these dependencies using `pip`:

```sh
pip install Flask Flask-SQLAlchemy uuid dotenv pandas numpy
```

## Running the Server

Please note that this server is for reference only and is not intended to be run as part of the main project. If you still wish to run it, you can use the following commands:

1. Set up the environment variables in a `.env` file.
2. Run the server:

```sh
python server.py
```

The server will start and listen for incoming requests on `127.0.0.1:5000`.

## API Endpoints

- `GET /test`: A test endpoint to verify the server is running.
- `GET /search?query=<search_text>`: Search for records based on the provided search text.
- `GET /search-results?query_id=<search_id>&page=<page>`: Retrieve paginated search results based on the query ID.

## Note

This implementation is provided for reference only and may not be fully functional or up-to-date with the main Rust implementation. Additionally, it is not fully implemented yet.

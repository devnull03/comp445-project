# Search Engine Project

## Project Description
This project is a web application designed to perform searches and display results with similarity information. It allows users to search for documents, view detailed information about each document, and see similar documents.

## Setup Instructions

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

## Routes
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

## Models
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

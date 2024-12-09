use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bin::processing::Record;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResultsRes {
    pub search_id: Uuid,
    pub data: Vec<RecordResponse>,
    pub number_of_results: u32,
    pub page: u32,
    pub total_pages: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecordResponse {
    pub data: Record,
    pub similar_docs: Vec<SimilarityInfoFull>,
}

#[derive(Deserialize, Debug)]
pub struct SimilarInfo {
    pub doc_id: i32,
    pub similarity: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SimilarityInfoFull {
    pub doc: Record,
    pub similarity: f32,
}
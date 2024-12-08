use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bin::processing::Record;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResultsRes {
    pub search_id: Uuid,
    pub data: Vec<Record>,
    pub number_of_results: u32,
    pub page: u32,
    pub total_pages: u32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct RecordResponse {
	pub data: Record,
	pub similar_docs: Vec<u32>,
}


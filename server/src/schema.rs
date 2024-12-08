use serde::{Deserialize, Serialize};
use uuid::Uuid;

// -- /api/search?query=<search_text>
#[derive(Debug, Deserialize)]
pub struct SearchReq {
    pub search_text: Option<String>,
}

// -- /api/search-results?query_id=<search_id>&page=<page>
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResultsReq {
    pub query_id: Uuid,
    pub page: u32,
}

// -- /api/get-record?id=<id>
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordReq {
    pub id: u32,
}

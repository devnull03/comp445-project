use axum::{
    extract::{Query, State},
    // http::StatusCode,
    response::IntoResponse,
    Json,
};
use rusqlite::params;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use uuid::Uuid;

use crate::{
    bin::processing::Record,
    model::{RecordResponse, SearchResultsRes, SimilarInfo, SimilarityInfoFull},
    schema::SearchReq,
    AppState, QueryState,
};

const QUERY_LIMIT: i32 = 10;

pub async fn search_handler(
    query: Query<SearchReq>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    let search_words = query.search_text.to_owned().unwrap_or("".to_string());

    let search_words = search_words
        .split_whitespace()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let search_entries = data
        .db
        .call(|conn| {
            let mut stmt_inv_idx =
                conn.prepare("SELECT entries FROM inverse_index WHERE string LIKE (?1)")?;
            let mut inv_idx_result_set = HashSet::<String>::new();

            for word in search_words {
                let rows = stmt_inv_idx
                    .query_map(params![format!("%{}%", word)], |row| {
                        Ok(row
                            .get::<_, String>(0)
                            .unwrap()
                            .strip_prefix("[")
                            .unwrap()
                            .strip_suffix("]")
                            .unwrap()
                            .split(", ")
                            // .map(|e| e.parse::<u32>().unwrap())
                            .map(|e| e.to_string())
                            .collect::<Vec<String>>())
                    })
                    .unwrap()
                    .flatten()
                    .flatten()
                    .collect::<Vec<_>>();

                for r in rows {
                    inv_idx_result_set.insert(r);
                }
            }

            let mut stmt_records = conn.prepare("SELECT * FROM records WHERE id = (?1)")?;
            let mut records_result_set: HashMap<u32, Record> = HashMap::new();
            let mut extended_records_result_set: Vec<RecordResponse> = Vec::new();

            let mut stmt_similar_docs = conn
                .prepare("SELECT similar_documents FROM similarities WHERE document_id = (?1)")?;

            for id in &inv_idx_result_set {
                let record_rows = stmt_records
                    .query_map(params![id], |row| Ok(Record::from(row)))
                    .unwrap()
                    .flatten()
                    .collect::<Vec<Record>>();

                records_result_set.insert(record_rows[0].id, record_rows[0].clone());

                let similarity_rows = stmt_similar_docs
                    .query_map(params![id], |row| {
                        Ok(row
                            .get::<_, String>(0)
                            .unwrap()
                            .strip_prefix("[{")
                            .unwrap()
                            .strip_suffix("},]")
                            .unwrap()
                            .split("},{")
                            .map(|e: &str| {
                                // println!("{{{}}}", e);
                                serde_json::from_str::<SimilarInfo>(&format!("{{{}}}", e)).unwrap()
                            })
                            .collect::<Vec<SimilarInfo>>())
                    })
                    .unwrap()
                    .flatten()
                    .flatten()
                    .collect::<Vec<_>>();

                let mut similarity_info: Vec<SimilarityInfoFull> = Vec::new();

                let mut count = 0;
                for r in similarity_rows {
                    if records_result_set.contains_key(&(r.doc_id as u32)) {
                        similarity_info.push(SimilarityInfoFull {
                            doc: records_result_set.get(&(r.doc_id as u32)).unwrap().clone(),
                            similarity: r.similarity,
                        });
                    } else {
                        let new_record = stmt_records
                            .query_map(params![r.doc_id], |row| Ok(Record::from(row)))
                            .unwrap()
                            .flatten()
                            .collect::<Vec<Record>>()
                            .pop()
                            .unwrap();

                        similarity_info.push(SimilarityInfoFull {
                            doc: new_record,
                            similarity: r.similarity,
                        });
                    }
                    count += 1;
                    if count >= QUERY_LIMIT {
                        break;
                    }
                }
                extended_records_result_set.push(RecordResponse {
                    data: record_rows[0].clone(),
                    similar_docs: similarity_info,
                });
            }

            Ok(extended_records_result_set)
        })
        .await
        .unwrap();

    let new_search_id = Uuid::new_v4();
    data.cached_queries.lock().unwrap().insert(
        new_search_id,
        QueryState {
            text_query: query.search_text.clone().unwrap(),
            data: search_entries.clone(),
        },
    );

    Json(SearchResultsRes {
        search_id: new_search_id,
        data: search_entries.get(0..10).unwrap().to_vec(),
        number_of_results: search_entries.len() as u32,
        page: 1,
        total_pages: search_entries.len().div_ceil(QUERY_LIMIT as usize) as u32,
    })
}

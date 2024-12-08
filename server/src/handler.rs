use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rusqlite::params;
use std::{collections::HashSet, sync::Arc};

use crate::{schema::SearchReq, AppState};

pub async fn search_handler(
    query: Query<SearchReq>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    let search_words = query.search_text.to_owned().unwrap_or("".to_string());

    let search_words = search_words
        .split_whitespace()
        .map(|v| v.to_string())
        // .into_iter()
        .collect::<Vec<String>>();

    let search_entries = data
        .db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT entries FROM inverse_index WHERE string = (?1)")?;
            let mut result_set = HashSet::<u32>::new();

            for word in search_words {
                let rows = stmt
                    .query_map(params![word], |row| {
                        Ok(row
                            .get::<_, String>(0)
                            .unwrap()
                            .strip_prefix("[")
                            .unwrap()
                            .strip_suffix("]")
                            .unwrap()
                            .split(", ")
                            .map(|e| e.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>())
                    })
                    .unwrap()
                    .flatten()
                    .flatten()
                    .collect::<Vec<_>>();

                for r in rows {
                    result_set.insert(r);
                }
            }

            Ok(result_set)
        })
        .await
        .unwrap();

    Json(search_entries)
}

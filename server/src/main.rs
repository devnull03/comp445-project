pub mod bin;
pub mod search;

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};
use bin::processing::Record;
use search::{build_db, search_db};
use serde::Deserialize;
use uuid::Uuid;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use uuid::Uuid;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let data: Arc<
        RwLock<(
            HashMap<u32, bin::processing::Record>,
            HashMap<String, HashSet<u32>>,
        )>,
    > = Arc::new(RwLock::new(build_db("".to_string())));

    // Compose the routes
    let app = Router::new()
        .route("/search", routing::get(lookup))
        // .route("/search-results", routing::get())
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(data);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Search {
    pub search_text: Option<String>,
}

pub struct SearchResults {
    pub query_id: Uuid,
    pub data: Vec<Record>,
    
}

async fn lookup(search: Option<Query<Search>>, State(db): State<Data>) -> impl IntoResponse {
    let Query(search) = search.unwrap_or_default();
    let db = db.read().unwrap();

    // search.search_text;
    let search_results =
        search_db(&search.search_text.unwrap_or("".to_string()), &db.1, &db.0).clone();

    let search_results = search_results.into_iter()
        .cloned()
        .collect::<Vec<_>>();

    Json(search_results)
}

type Data = Arc<
    RwLock<(
        HashMap<u32, bin::processing::Record>,
        HashMap<String, HashSet<u32>>,
    )>,
>;

// #[derive(Debug, Serialize, Clone)]
// struct Item {
//     id: Uuid,
//     title: String,
//     text: String,
//     truth: bool,
// }

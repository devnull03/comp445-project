pub mod bin;
pub mod handler;
pub mod model;
pub mod schema;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing, Router};
use dotenv::dotenv;
use handler::{search_handler, search_pagination_handler};
use model::RecordResponse;
use tokio_rusqlite;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

pub struct AppState {
    db: tokio_rusqlite::Connection,
    cached_queries: Mutex<HashMap<Uuid, QueryState>>,
}

#[allow(dead_code)]
pub struct QueryState {
    text_query: String,
    data: Vec<RecordResponse>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_file_name = std::env::var("DB_FILE_PATH").unwrap_or("processed.db".to_string());
    let conn = tokio_rusqlite::Connection::open(&database_file_name)
        .await
        .unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Compose the routes
    let app = Router::new()
        .route("/test", routing::get(|| async { "this is a test" }))
        .route("/search", routing::get(search_handler))
        .route("/search-results", routing::get(search_pagination_handler))
        .with_state(Arc::new(AppState {
            db: conn.clone(),
            cached_queries: Mutex::new(HashMap::new()),
        }))
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
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

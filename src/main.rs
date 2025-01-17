use std::sync::Arc;

use anyhow::Result;
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Router,
};
use db::{delete_list, new_db, update_list, write_list};
use firebase_auth::{FirebaseAuth, FirebaseAuthState};
use firestore::FirestoreDb;
use handlers::{auth, index};
use uuid::Uuid;
use views::IndexTemplate;

mod db;
mod handlers;
mod views;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<FirebaseAuthState>,
    pub db: Arc<FirestoreDb>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = new_db().await?;
    let firebase_auth = FirebaseAuth::new("r-j-magenta-carrot-42069").await;

    let app_state = AppState {
        auth: Arc::new(FirebaseAuthState::new(firebase_auth)),
        db: Arc::new(db),
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    let app = Router::new()
        .route("/", get(index))
        // .route("/auth", get(auth))
        // .route("/update", post(update_list))
        // .route("/delete", post(delete_list))
        .route("/lists/create", post(write_list))
        // .route("/todo", post(add_todo))
        // .route("/todo/{id}", delete(remove_todo).patch(toggle_todo))
        .with_state(app_state);

    axum::serve(listener, app).await?;

    Ok(())
}

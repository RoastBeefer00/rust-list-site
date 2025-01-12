use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
use db::{new_db, FirebaseUser};
use firebase_auth::{FirebaseAuth, FirebaseAuthState};
use firestore::FirestoreDb;
// use handlers::{add_todo, auth, remove_todo, toggle_todo};
use handlers::auth;
use tokio::sync::RwLock;
use uuid::Uuid;
use views::IndexTemplate;

mod db;
mod handlers;
mod views;

#[derive(Clone)]
pub struct AppState {
    pub auth: FirebaseAuthState,
    pub db: FirestoreDb,
}

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub complete: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = new_db().await?;
    let firebase_auth = FirebaseAuth::new("r-j-magenta-carrot-42069").await;

    let app_state = AppState {
        auth: FirebaseAuthState::new(firebase_auth),
        db,
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    let app = Router::new()
        .route("/", get(IndexTemplate::render))
        .route("/auth", get(auth))
        // .route("/todo", post(add_todo))
        // .route("/todo/{id}", delete(remove_todo).patch(toggle_todo))
        .with_state(app_state);

    axum::serve(listener, app).await?;

    Ok(())
}

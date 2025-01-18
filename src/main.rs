use anyhow::Result;
use axum::extract::FromRef;
use axum::{
    routing::{get, post},
    Router,
};
use db::new_db;
use firebase_auth::{FirebaseAuth, FirebaseAuthState};
use firestore::FirestoreDb;
use handlers::index;
use views::{List, ListGroup};

mod db;
mod handlers;
mod views;

#[derive(Clone)]
pub struct AppState {
    pub auth: FirebaseAuthState,
    pub db: FirestoreDb,
}

impl FromRef<AppState> for FirestoreDb {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
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
        .route("/", get(index))
        .route("/groups", get(ListGroup::get))
        // .route("/auth", get(auth))
        // .route("/update", post(update_list))
        // .route("/delete", post(delete_list))
        .route("/lists/create", post(List::write))
        // .route("/todo", post(add_todo))
        // .route("/todo/{id}", delete(remove_todo).patch(toggle_todo))
        .with_state(app_state);

    axum::serve(listener, app).await?;

    Ok(())
}

use axum::{
    routing::{delete, get, post},
    Router,
};
use firebase_auth::{FirebaseAuth, FirebaseAuthState};
use handlers::{add_todo, auth, remove_todo, toggle_todo};
use tokio::sync::RwLock;
use uuid::Uuid;
use views::IndexTemplate;

mod auth;
mod handlers;
mod views;

pub struct AppState {
    todos: RwLock<Vec<Todo>>,
}

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub complete: bool,
}

#[tokio::main]
async fn main() {
    let firebase_auth = FirebaseAuth::new("r-j-magenta-carrot-42069").await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let app = Router::new()
        .route("/", get(IndexTemplate::render))
        .route("/auth", get(auth))
        // .route("/todo", post(add_todo))
        // .route("/todo/{id}", delete(remove_todo).patch(toggle_todo))
        .with_state(FirebaseAuthState::new(firebase_auth));

    axum::serve(listener, app).await.unwrap();
}

use anyhow::Result;
use axum::extract::FromRef;
use axum::routing::{delete, put};
use axum::{
    routing::{get, post},
    Router,
};
use db::new_db;
use firebase_auth::{FirebaseAuth, FirebaseAuthState};
use firestore::FirestoreDb;
use handlers::index;
use views::{List, ListGroup, ListItem};

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
        .route("/groups", get(ListGroup::get_view))
        .route("/list", post(List::write_view))
        .route("/list/{id}", get(List::get_view).delete(List::delete_view))
        .route(
            "/list/{id}/complete",
            delete(List::remove_all_complete_view),
        )
        .route("/list/{id}/item", post(ListItem::write_view))
        .route(
            "/list/{list_id}/item/{item_id}",
            get(ListItem::get_view)
                .put(ListItem::update_view)
                .delete(ListItem::delete_view),
        )
        .route(
            "/list/{list_id}/item/{item_id}/edit",
            get(ListItem::form_view),
        )
        .route(
            "/list/{list_id}/item/{item_id}/toggle",
            put(ListItem::toggle_view),
        )
        .with_state(app_state);

    axum::serve(listener, app).await?;

    Ok(())
}

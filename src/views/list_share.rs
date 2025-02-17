use crate::db::{FirebaseUser, User};
use askama_axum::Template;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use firestore::FirestoreDb;
use serde::{Deserialize, Serialize};

// A list
// Will be a single document in the list_items collection
#[derive(Debug, Clone, Serialize, Template, Deserialize)]
#[template(path = "list_share.html")]
pub struct ListShare {
    pub users: Vec<User>,
}

impl ListShare {
    pub fn new(users: Vec<User>) -> Self {
        ListShare { users }
    }

    pub async fn view(user: FirebaseUser, State(db): State<FirestoreDb>) -> impl IntoResponse {
        let user = User::from(&user);
        match User::get_all(&db).await {
            Ok(mut users) => {
                users.iter_mut().filter(|u| u.id != user.id);
                ListShare::new(users).into_response()
            }
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

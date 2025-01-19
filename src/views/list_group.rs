use crate::db::User;
use crate::AppState;

use super::List;
use askama_axum::Template;
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
// use firebase_auth::FirebaseUser;
use crate::db::FirebaseUser;
use firestore::FirestoreDb;

// A grouping of lists
// Meant to be grouped by owner
#[derive(Debug, Clone, Template)]
#[template(path = "list_group.html")]
pub struct ListGroup {
    pub owner: String,
    pub lists: Vec<List>,
}

impl ListGroup {
    pub fn new(owner: String, lists: Vec<List>) -> Self {
        ListGroup { owner, lists }
    }

    pub async fn get_view(fb_user: FirebaseUser, State(db): State<FirestoreDb>) -> Response {
        let user_name = fb_user
            .clone()
            .name
            .unwrap_or_else(|| fb_user.user_id.clone());
        let user = User::from(&fb_user);
        let db_user = user.get(&db).await.unwrap();
        match db_user {
            Some(user) => match user.get_all_lists(&db).await {
                Ok(lists) => ListGroup {
                    owner: user.name.unwrap_or_else(|| user.id),
                    lists,
                }
                .into_response(),
                Err(e) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            },
            None => {
                user.create(&db).await.unwrap();
                return ListGroup {
                    owner: user_name,
                    lists: vec![],
                }
                .into_response();
            }
        }
    }
}

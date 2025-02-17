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

#[derive(Debug, Clone, Template)]
#[template(path = "list_groups.html")]
pub struct ListGroups {
    pub groups: Vec<ListGroup>,
}

impl ListGroups {
    pub async fn get_view(fb_user: FirebaseUser, State(db): State<FirestoreDb>) -> Response {
        let user = User::from(&fb_user);
        let db_user = user.get(&db).await.unwrap();
        match db_user {
            Some(user) => match user.get_all_list_groups(&db).await {
                Ok(mut groups) => {
                    if let Some(email) = fb_user.email {
                        groups.iter_mut().for_each(|group| {
                            if group.owner == email {
                                group.owner = "My".to_string();
                            }
                        });
                    }
                    ListGroups { groups }.into_response()
                }
                Err(e) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            },
            None => {
                user.create(&db).await.unwrap();
                return ListGroup {
                    owner: "My".to_string(),
                    lists: vec![],
                }
                .into_response();
            }
        }
    }
}

// A grouping of lists
// Meant to be grouped by owner
#[derive(Debug, Clone, Template)]
#[template(path = "macros/list_group.html")]
pub struct ListGroup {
    pub owner: String,
    pub lists: Vec<List>,
}

impl ListGroup {
    pub fn new(owner: String, lists: Vec<List>) -> Self {
        ListGroup { owner, lists }
    }
}

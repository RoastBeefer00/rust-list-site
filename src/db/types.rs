use askama_axum::Template;
use firebase_auth::FirebaseUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// A list
// Will be a single document in the list_items collection
#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "list.html")]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "list_item.html")]
pub struct ListItem {
    pub id: Uuid,
    pub text: String,
    pub complete: bool,
}

// A user from the site
// Contains the lists the user has access to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub lists: Vec<List>,
}

impl From<FirebaseUser> for User {
    fn from(user: FirebaseUser) -> Self {
        User {
            id: user.user_id,
            name: user.name,
            email: user.email,
            lists: vec![],
        }
    }
}

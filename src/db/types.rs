use std::sync::Arc;

use super::auth::FirebaseUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// A user from the site
// Contains the lists the user has access to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub lists: Vec<Uuid>,
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

impl From<Arc<FirebaseUser>> for User {
    fn from(user: Arc<FirebaseUser>) -> Self {
        User {
            id: user.user_id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            lists: vec![],
        }
    }
}

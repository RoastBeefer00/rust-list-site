use firebase_auth::FirebaseUser;
use crate::views::List;

// A user from the site
// Contains the lists the user has access to
#[derive(Debug, Clone)]
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

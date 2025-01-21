use crate::views::IndexTemplate;
use firebase_auth::FirebaseUser;
use maud::Markup;
use maud::Render;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AddTodo {
    text: String,
}

pub async fn auth(user: FirebaseUser) -> String {
    format!(
        "Hello, {} with email: {}!",
        user.name.unwrap_or("Anonymous".to_string()),
        user.email.unwrap_or("no email".to_string())
    )
}

pub async fn index() -> Markup {
    IndexTemplate { groups: Vec::new() }.render()
}

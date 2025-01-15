use askama_axum::Template;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "list_item.html")]
pub struct ListItem {
    pub id: Uuid,
    pub text: String,
    pub complete: bool,
}

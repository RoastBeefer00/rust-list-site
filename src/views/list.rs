use askama_axum::Template;
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

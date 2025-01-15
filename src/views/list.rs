use askama_axum::Template;
use super::list_item::ListItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// A list
// Will be a single document in the list_items collection
#[derive(Debug, Clone, Serialize, Template, Deserialize)]
#[template(path = "list.html")]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub items: Vec<ListItem>,
}

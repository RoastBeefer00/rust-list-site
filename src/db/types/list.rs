use crate::views::ListItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub items: Vec<ListItem>,
}

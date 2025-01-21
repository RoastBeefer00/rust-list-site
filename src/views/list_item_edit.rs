use askama_axum::Template;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ListItem;

#[derive(Deserialize)]
pub struct ListItemEditForm {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "macros/list_item_edit.html")]
pub struct ListItemEdit {
    pub list_id: Uuid,
    pub item: ListItem,
}

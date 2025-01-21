use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ListItem;

#[derive(Deserialize)]
pub struct ListItemEditForm {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemEdit {
    pub list_id: Uuid,
    pub item: ListItem,
}

impl Render for ListItemEdit {
    fn render(&self) -> Markup {
        html! {}
    }
}

use askama_axum::Template;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Template, Deserialize, Serialize)]
#[template(path = "macros/list_preview.html")]
pub struct ListPreview {
    pub id: Uuid,
    pub name: String,
    pub shareable: bool,
}

impl From<super::List> for ListPreview {
    fn from(list: super::List) -> Self {
        ListPreview {
            id: list.id,
            name: list.name,
            shareable: false,
        }
    }
}

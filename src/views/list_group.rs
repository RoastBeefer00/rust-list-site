use super::list::List;
use askama_axum::Template;
use serde::{Deserialize, Serialize};

// A grouping of lists
// Meant to be grouped by owner
#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "list_group.html")]
pub struct ListGroup {
    pub owner: String,
    pub lists: Vec<List>,
}

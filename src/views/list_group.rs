use super::list::List;
use askama_axum::Template;

// A grouping of lists
// Meant to be grouped by owner
#[derive(Debug, Clone, Template)]
#[template(path = "list_group.html")]
pub struct ListGroup {
    pub owner: String,
    pub lists: Vec<List>,
}

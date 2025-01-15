use super::list_group::ListGroup;
use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub todos: Vec<ListGroup>,
}

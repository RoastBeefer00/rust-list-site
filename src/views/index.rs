use super::list_group::ListGroup;
use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub todos: &'a [ListGroup],
}

impl IndexTemplate<'_> {
    pub async fn render() -> Response {
        // let state = state.todos.read().await;
        let todos = IndexTemplate { todos: &Vec::new() };
        todos.into_response()
    }
}

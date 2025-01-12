use crate::{AppState, Todo};
use askama_axum::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
};
use std::sync::Arc;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub todos: &'a [Todo],
}

impl IntoResponse for IndexTemplate<'_> {
    fn into_response(self) -> Response {
        Html(self.render().unwrap()).into_response()
    }
}

impl IndexTemplate<'_> {
    pub async fn render() -> Response {
        // let state = state.todos.read().await;
        let todos = IndexTemplate { todos: &Vec::new() };
        todos.into_response()
    }
}

#[derive(Template)]
#[template(path = "todo.html")]
pub struct TodoTemplate<'a> {
    pub todo: &'a Todo,
}

impl IntoResponse for TodoTemplate<'_> {
    fn into_response(self) -> Response {
        Html(self.render().unwrap()).into_response()
    }
}

#[derive(Template)]
#[template(path = "todo_text.html")]
pub struct TodoTextTemplate<'a> {
    pub todo: &'a Todo,
}

impl IntoResponse for TodoTextTemplate<'_> {
    fn into_response(self) -> Response {
        Html(self.render().unwrap()).into_response()
    }
}

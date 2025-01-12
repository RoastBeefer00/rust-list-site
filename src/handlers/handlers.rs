use std::sync::Arc;

use crate::db::FirebaseUser;
use crate::views::{TodoTemplate, TodoTextTemplate};
use crate::{AppState, Todo};
use axum::{
    debug_handler,
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct AddTodo {
    text: String,
}

// #[debug_handler]
// pub async fn add_todo(State(state): State<Arc<AppState>>, Form(form): Form<AddTodo>) -> Response {
//     let todo_text = form.text;
//     let mut state = state.todos.write().await;
//     let todo = Todo {
//         id: Uuid::new_v4(),
//         text: todo_text,
//         complete: false,
//     };
//     state.push(todo.clone());
//     TodoTemplate { todo: &todo }.into_response()
// }
//
// #[debug_handler]
// pub async fn remove_todo(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Response {
//     let mut todos = state.todos.write().await;
//     todos.retain(|todo| todo.id != id);
//     (StatusCode::OK).into_response()
// }
//
// #[debug_handler]
// pub async fn toggle_todo(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Response {
//     let mut todos = state.todos.write().await;
//     let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) else {
//         return (StatusCode::NOT_FOUND).into_response();
//     };
//     todo.complete = !todo.complete;
//     TodoTextTemplate { todo }.into_response()
// }

pub async fn auth(user: FirebaseUser) -> String {
    format!(
        "Hello, {} with email: {}!",
        user.name.unwrap_or("Anonymous".to_string()),
        user.email.unwrap_or("no email".to_string())
    )
}

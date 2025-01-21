use super::{ListItemEdit, ListItemEditForm};
use crate::db::List;
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use firestore::FirestoreDb;
use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub list_id: Uuid,
    pub id: Uuid,
    pub text: String,
    pub complete: bool,
}

impl Render for ListItem {
    fn render(&self) -> Markup {
        html! {
            li hx-target="this" hx-swap="outerHTML" class="flex items-center space-x-4 p-4 border-b border-gray-700 bg-gray-800 hover:bg-gray-700 rounded-lg shadow-sm" {
                input type="checkbox" id="{{ item.id }}" class="form-checkbox h-6 w-6 text-blue-400" hx-put="/list/{{ id }}/item/{{ item.id }}/toggle" hx-target="next span" hx-swap="outerHTML" checked=(self.complete) {}
                (self.text().render())
                button hx-get="/list/{{ id }}/item/{{ item.id }}/edit" class="text-blue-400 hover:text-blue-600 font-semibold" { "Edit" }
                button hx-delete="/list/{{ id }}/item/{{ item.id }}" class="text-red-400 hover:text-blue-600 font-semibold" { "Delete" }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct ListItemCreateForm {
    pub text: String,
}

impl ListItem {
    pub fn new(list_id: Uuid, text: String) -> Self {
        ListItem {
            list_id,
            id: Uuid::new_v4(),
            text,
            complete: false,
        }
    }

    pub fn text(&self) -> Markup {
        html! {}
    }

    pub async fn write_view(
        State(db): State<FirestoreDb>,
        Path(list_id): Path<Uuid>,
        Form(form): Form<ListItemCreateForm>,
    ) -> Response {
        let mut list = List::get(list_id, &db).await.unwrap();
        let item = ListItem::new(list_id, form.text.clone());
        list.items.push(item.clone());
        if let Err(e) = List::update(&list, &db).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        } else {
            item.render().into_response()
        }
    }

    pub async fn get_view(
        State(db): State<FirestoreDb>,
        Path((list_id, item_id)): Path<(Uuid, Uuid)>,
    ) -> Response {
        match db
            .fluent()
            .select()
            .by_id_in("lists")
            .obj::<List>()
            .one(&list_id.to_string())
            .await
        {
            Ok(res) => match res {
                Some(list) => {
                    let item = list.items.iter().find(|item| item.id == item_id);
                    match item {
                        Some(item) => item.clone().render().into_response(),
                        None => (StatusCode::NOT_FOUND).into_response(),
                    }
                }
                None => (StatusCode::NOT_FOUND).into_response(),
            },
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }

    pub async fn form_view(
        State(db): State<FirestoreDb>,
        Path((list_id, item_id)): Path<(Uuid, Uuid)>,
    ) -> Response {
        let list = List::get(list_id, &db).await.unwrap();
        let item = list.items.iter().find(|item| item.id == item_id);
        match item {
            Some(item) => {
                let form = ListItemEdit {
                    list_id,
                    item: item.clone(),
                };
                form.render().into_response()
            }
            None => (StatusCode::NOT_FOUND).into_response(),
        }
    }

    pub async fn update_view(
        State(db): State<FirestoreDb>,
        Path((list_id, item_id)): Path<(Uuid, Uuid)>,
        Form(form): Form<ListItemEditForm>,
    ) -> Response {
        let mut list = List::get(list_id, &db).await.unwrap();
        let item = match list.items.iter_mut().find(|item| item.id == item_id) {
            Some(item) => {
                item.text = form.name.clone();
                item.clone()
            }
            None => return (StatusCode::NOT_FOUND).into_response(),
        };
        if let Err(e) = List::update(&list, &db).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        } else {
            item.render().into_response()
        }
    }

    pub async fn toggle_view(
        State(db): State<FirestoreDb>,
        Path((list_id, item_id)): Path<(Uuid, Uuid)>,
    ) -> Response {
        let mut list = List::get(list_id, &db).await.unwrap();
        let item = match list.items.iter_mut().find(|item| item.id == item_id) {
            Some(item) => {
                item.complete = !item.complete;
                item.clone()
            }
            None => return (StatusCode::NOT_FOUND).into_response(),
        };
        if let Err(e) = List::update(&list, &db).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        } else {
            item.text().into_response()
        }
    }

    pub async fn delete_view(
        State(db): State<FirestoreDb>,
        Path((list_id, item_id)): Path<(Uuid, Uuid)>,
    ) -> Response {
        let mut list = List::get(list_id, &db).await.unwrap();
        let item = match list.items.iter().position(|item| item.id == item_id) {
            Some(index) => list.items.remove(index),
            None => return (StatusCode::NOT_FOUND).into_response(),
        };
        if let Err(e) = List::update(&list, &db).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        } else {
            item.render().into_response()
        }
    }
}

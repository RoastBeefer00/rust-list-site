use askama_axum::Template;
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use firestore::FirestoreDb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{List, ListItemText};
use super::{ListItemEdit, ListItemEditForm};

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "list_item.html")]
pub struct ListItem {
    pub list_id: Uuid,
    pub id: Uuid,
    pub text: String,
    pub complete: bool,
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
            item.into_response()
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
                        Some(item) => item.clone().into_response(),
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
                form.into_response()
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
            item.into_response()
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
            ListItemText {
                complete: item.complete,
                text: item.text,
            }
            .into_response()
        }
    }

    pub async fn delete_view(
        State(db): State<FirestoreDb>,
        Path((list_id, item_id)): Path<(Uuid, Uuid)>,
    ) -> Response {
        let mut list = List::get(list_id, &db).await.unwrap();
        match list.items.iter().position(|item| item.id == item_id) {
            Some(index) => list.items.remove(index),
            None => return (StatusCode::NOT_FOUND).into_response(),
        };
        if let Err(e) = List::update(&list, &db).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        } else {
            (StatusCode::OK).into_response()
        }
    }
}

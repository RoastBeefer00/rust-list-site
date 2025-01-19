use askama_axum::Template;
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use firestore::FirestoreDb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::List;
use super::{ListItemEdit, ListItemEditForm};

#[derive(Debug, Clone, Serialize, Deserialize, Template)]
#[template(path = "list_item.html")]
pub struct ListItem {
    pub id: Uuid,
    pub text: String,
    pub complete: bool,
}

impl ListItem {
    pub fn new(text: String) -> Self {
        ListItem {
            id: Uuid::new_v4(),
            text,
            complete: false,
        }
    }

    pub async fn get(
        State(db): State<FirestoreDb>,
        Path(list_id): Path<Uuid>,
        Path(item_id): Path<Uuid>,
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

    pub async fn get_edit_form(
        State(db): State<FirestoreDb>,
        Path(list_id): Path<Uuid>,
        Path(item_id): Path<Uuid>,
    ) -> Response {
        let list = List::get_view(list_id, &db).await.unwrap();
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

    pub async fn edit(
        State(db): State<FirestoreDb>,
        Form(form): Form<ListItemEditForm>,
        Path(list_id): Path<Uuid>,
        Path(item_id): Path<Uuid>,
    ) -> Response {
        let list = List::get_view(list_id, &db).await.unwrap();
        let item = list.items.iter().find(|item| item.id == item_id);
        match item {
            Some(item) => {
                let mut list = list.clone();
                list.items = list
                    .items
                    .iter()
                    .map(|i| {
                        if i.id == item_id {
                            ListItem {
                                id: i.id,
                                text: form.name.clone(),
                                complete: i.complete,
                            }
                        } else {
                            i.clone()
                        }
                    })
                    .collect();
                List::update_view(user, db).await;
                (StatusCode::OK).into_response()
            }
            None => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}

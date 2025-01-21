use std::sync::Arc;

use crate::db::FirebaseUser;
use crate::db::List;
use crate::db::User;
use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use firestore::FirestoreDb;
use maud::{html, Markup, Render};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateListForm {
    pub name: String,
}

#[derive(Deserialize)]
struct UpdateListForm {
    text: String,
}

impl Render for List {
    fn render(&self) -> Markup {
        html! {
            div class="max-w-2xl mx-auto p-4" id={ "list-" (self.id)} {
                h1 class="text-2xl font-bold mb-4" { (self.name) }
                div class="mb-4" {
                    form hx-post={ "/list/" (self.id) "/item" } hx-target="#list-items-{{ id }}" hx-swap="beforeend" class="flex space-x-2" {
                        input type="text" name="text" placeholder="Add something to the list..." class="flex-grow p-2 border border-gray-300 rounded text-black" {}
                        button class="border border-black p-2 text-white bg-green-700 rounded" { "Add" }
                    }
                }
                div {
                    ul id={ "list-items-" (self.id) } {
                        @for item in &self.items {
                            (item.render())
                        }
                    }
                }
                button hx-delete={ "list/" (self.id) "/complete" } hx-target="#list-{{ id }}" hx-swap="outerHTML" class="p-2 border border-black text-white rounded mt-4 bg-red-700" { "Remove All Completed" }
            }
        }
    }
}

impl List {
    pub fn new(name: String, owner: String) -> Self {
        List {
            id: Uuid::new_v4(),
            name,
            owner,
            items: vec![],
        }
    }

    pub fn preview(&self) -> Markup {
        html! {
            li hx-get={ "/list/" (self.id) } hx-target="#body" hx-swap="innerHTML" class="flex items-center justify-between bg-gray-800 text-white rounded-lg px-4 py-2 mt-2 hover:bg-gray-700 cursor-pointer" {
                div class="w-full" {
                    span{ (self.name) }
                }
                button hx-delete={ "/list/" (self.id) } hx-target="closest li" hx-swap="outerHTML" class="border border-black p-2 text-white bg-red-700 hover:bg-red-800" onclick="event.stopPropagation()" {
                    "Delete"
                }
            }
        }
    }

    pub async fn get(id: Uuid, db: &FirestoreDb) -> Result<List> {
        db.fluent()
            .select()
            .by_id_in("lists")
            .obj()
            .one(&id.to_string())
            .await
            .unwrap()
            .context("Failed to get list")
    }

    pub async fn write(&self, db: &FirestoreDb) -> Result<List> {
        db.fluent()
            .insert()
            .into("lists")
            .document_id(self.id.to_string())
            .object(self)
            .execute::<List>()
            .await
            .context("Error writing list")
    }

    pub async fn get_view(State(db): State<FirestoreDb>, Path(id): Path<Uuid>) -> Response {
        match Self::get(id, &db).await {
            Ok(list) => list.render().into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    pub async fn write_view(
        user: FirebaseUser,
        State(db): State<FirestoreDb>,
        Form(form): Form<CreateListForm>,
    ) -> impl IntoResponse {
        let list = List {
            id: Uuid::new_v4(),
            name: form.name,
            owner: user.clone().user_id,
            items: vec![],
        };
        let db = Arc::new(db);
        let user = Arc::new(User::from(user));
        let list_clone = list.clone();
        let create_list_future = {
            let db = db.clone();
            tokio::spawn(async move { list_clone.write(&db).await })
        };
        let get_user_future = {
            let db = db.clone();
            let user = user.clone();
            tokio::spawn(async move { user.get(&db).await })
        };
        if let Err(e) = create_list_future.await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
        match get_user_future.await {
            Ok(result) => match result {
                Ok(opt) => {
                    if let Some(mut user) = opt {
                        if let Err(e) = user.grant_access(list.id, &db).await {
                            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                                .into_response();
                        }
                    } else {
                        match user.create(&db).await {
                            Ok(mut user) => {
                                if let Err(e) = user.grant_access(list.id, &db).await {
                                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                                        .into_response();
                                }
                            }
                            Err(e) => {
                                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                                    .into_response()
                            }
                        }
                    }
                }
                Err(e) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                }
            },
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
        list.preview().into_response()
    }

    pub async fn update(&self, db: &FirestoreDb) -> Result<List> {
        db.fluent()
            .update()
            .in_col("lists")
            .document_id(self.id.to_string())
            .object(self)
            .execute::<List>()
            .await
            .context("Error updating list")
    }

    pub async fn delete_view(
        user: FirebaseUser,
        db: FirestoreDb,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        match db
            .fluent()
            .delete()
            .from("lists")
            .document_id(id.to_string())
            .execute()
            .await
        {
            Ok(_) => {
                let user = User::from(user);
                if let Some(mut user) = user.get(&db).await.unwrap() {
                    if let Err(e) = user.remove_access(id, &db).await {
                        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                    } else {
                        (StatusCode::OK).into_response()
                    }
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, "User not found").into_response()
                }
            }
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    pub async fn remove_all_complete_view(
        State(db): State<FirestoreDb>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        let mut list = Self::get(id, &db).await.unwrap();
        list.items.retain(|item| !item.complete);
        match list.update(&db).await {
            Ok(_) => list.render().into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

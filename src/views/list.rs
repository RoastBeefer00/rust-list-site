use std::sync::Arc;

use crate::db::User;
use anyhow::{Context, Result};
use askama_axum::Template;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use firebase_auth::FirebaseUser;
use firestore::FirestoreDb;

use super::{list_item::ListItem, ListPreview};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateListForm {
    pub name: String,
}

#[derive(Deserialize)]
struct UpdateListForm {
    list_id: Uuid,
    text: String,
}

// A list
// Will be a single document in the list_items collection
#[derive(Debug, Clone, Serialize, Template, Deserialize)]
#[template(path = "list.html")]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub items: Vec<ListItem>,
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

    pub async fn write(
        user: FirebaseUser,
        db: State<FirestoreDb>,
        Form(form): Form<CreateListForm>,
    ) -> impl IntoResponse {
        let list = List {
            id: Uuid::new_v4(),
            name: form.name,
            owner: user.clone().user_id,
            items: vec![],
        };
        let db = Arc::new(db);
        let user = Arc::new(user);
        let list_clone = list.clone();
        let create_list_future = {
            let db = db.clone();
            tokio::spawn(async move {
                db.fluent()
                    .insert()
                    .into("lists")
                    .document_id(&list_clone.id.to_string())
                    .object(&list_clone)
                    .execute::<List>()
                    .await
            })
        };
        let get_user_future = {
            let db = db.clone();
            let user = user.clone();
            tokio::spawn(async move { User::get(&user, &db).await })
        };
        if let Err(e) = create_list_future.await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
        match get_user_future.await {
            Ok(result) => match result {
                Ok(opt) => {
                    if let Some(mut user) = opt {
                        user.lists.push(list.id);
                        if let Err(e) = User::update(&user, &db).await {
                            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                                .into_response();
                        }
                    } else {
                        match User::create(&user, &db).await {
                            Ok(mut user) => {
                                user.lists.push(list.id);
                                if let Err(e) = User::update(&user, &db).await {
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
        ListPreview::from(list).into_response()
    }

    pub async fn update(
        user: FirebaseUser,
        db: FirestoreDb,
        Form(form): Form<UpdateListForm>,
    ) -> impl IntoResponse {
        // let list_item = ListItem {
        //     id: Uuid::new_v4(),
        //     text: form.text,
        //     complete: false,
        // };
        // match db
        //     .fluent()
        //     .update()
        //     .in_col("lists")
        //     .document_id(&list.id.h db
        //     .fluent()
        //     .update()
        //     .in_col("lists")
        //     .document_id(&list.id.tto_string())
        //     .object(&list)
        //     .execute::<List>()
        //     .await
        // {
        //     Ok(_) => (StatusCode::CREATED).into_response(),
        //     Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        // }
        (StatusCode::OK).into_response()
    }

    pub async fn delete(_user: FirebaseUser, db: FirestoreDb) -> impl IntoResponse {
        match db
            .fluent()
            .delete()
            .from("lists")
            .document_id(69.to_string())
            .execute()
            .await
        {
            Ok(_) => (StatusCode::CREATED).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

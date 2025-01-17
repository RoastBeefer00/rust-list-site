use std::sync::Arc;

use anyhow::{anyhow, Result};
use axum::{
    extract::Form,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use firestore::FirestoreDb;
use serde::Deserialize;
use uuid::Uuid;

use super::FirebaseUser;
use crate::views::{List, ListItem};
use crate::{db::types::User, views::ListPreview};

#[derive(Deserialize)]
pub struct CreateListForm {
    pub name: String,
}

pub async fn write_list(
    user: Arc<FirebaseUser>,
    db: Arc<FirestoreDb>,
    Form(form): Form<CreateListForm>,
) -> Response {
    let list = List {
        id: Uuid::new_v4(),
        name: form.name,
        owner: user.user_id.clone(),
        items: vec![],
    };
    let create_list_future = {
        let db = db.clone();
        let list_clone = list.clone();
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
        tokio::spawn(async move { get_user(&user, &db).await })
    };
    if let Err(e) = create_list_future.await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }
    match get_user_future.await {
        Ok(result) => match result {
            Ok(opt) => {
                if let Some(mut user) = opt {
                    user.lists.push(list.id);
                    if let Err(e) = update_user(user, &db).await {
                        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                    }
                } else {
                    if let Err(e) = create_user(&user, &db).await {
                        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                    }
                    let mut user = User::from(user);
                    user.lists.push(list.id);
                    if let Err(e) = update_user(user, &db).await {
                        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                    }
                }
            }
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
    ListPreview::from(list).into_response()
}

struct UpdateListForm {
    list_id: Uuid,
    text: String,
}

pub async fn update_list(
    user: FirebaseUser,
    db: FirestoreDb,
    Form(form): Form<UpdateListForm>,
) -> Response {
    // let list_item = ListItem {
    //     id: Uuid::new_v4(),
    //     text: form.text,
    //     complete: false,
    // };
    // match db
    //     .fluent()
    //     .update()
    //     .in_col("lists")
    //     .document_id(&list.id.to_string())
    //     .object(&list)
    //     .execute::<List>()
    //     .await
    // {
    //     Ok(_) => (StatusCode::CREATED).into_response(),
    //     Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    // }
    (StatusCode::OK).into_response()
}

pub async fn delete_list(_user: FirebaseUser, db: FirestoreDb) -> Response {
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

pub async fn create_user(user: &FirebaseUser, db: &FirestoreDb) -> Result<()> {
    let user = User::from(user.clone());
    match db
        .fluent()
        .insert()
        .into("users")
        .document_id(&user.id)
        .object(&user)
        .execute::<User>()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

pub async fn get_user(user: &FirebaseUser, db: &FirestoreDb) -> Result<Option<User>> {
    match db
        .fluent()
        .select()
        .by_id_in("users")
        .obj()
        .one(&user.user_id)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

pub async fn update_user(user: User, db: &FirestoreDb) -> Result<()> {
    let user = User::from(user);
    match db
        .fluent()
        .update()
        .in_col("users")
        .document_id(&user.id)
        .object(&user)
        .execute::<User>()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

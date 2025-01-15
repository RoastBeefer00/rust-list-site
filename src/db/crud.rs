use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use firestore::FirestoreDb;
use uuid::Uuid;

use crate::views::{List, ListItem};
use super::FirebaseUser;

pub async fn write_list(user: FirebaseUser, db: FirestoreDb) -> Response {
    let list = List {
        id: Uuid::new_v4(),
        name: "My List".to_string(),
        owner: user.user_id,
        items: vec![ListItem {
            id: Uuid::new_v4(),
            text: "Item 1".to_string(),
            complete: false,
        }],
    };
    match db
        .fluent()
        .insert()
        .into("lists")
        .document_id(&list.id.to_string())
        .object(&list)
        .execute::<List>()
        .await
    {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update_list(user: FirebaseUser, db: FirestoreDb) -> Response {
    let list = List {
        id: Uuid::new_v4(),
        name: "My List".to_string(),
        owner: user.user_id,
        items: vec![ListItem {
            id: Uuid::new_v4(),
            text: "Item 2".to_string(),
            complete: false,
        }],
    };
    match db
        .fluent()
        .update()
        .in_col("lists")
        .document_id(&list.id.to_string())
        .object(&list)
        .execute::<List>()
        .await
    {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
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

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use firestore::FirestoreDb;

use super::firestore::List;
use super::FirebaseUser;

pub async fn write_list(user: FirebaseUser, db: FirestoreDb) -> Response {
    let list = List {
        id: 69,
        name: "My List".to_string(),
        owner: user.user_id,
        items: vec!["item1".to_string(), "item2".to_string()],
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
        id: 69,
        name: "My List".to_string(),
        owner: user.user_id,
        items: vec!["item1".to_string(), "item3".to_string()],
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

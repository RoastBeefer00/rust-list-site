use crate::AppState;
use anyhow::anyhow;
use anyhow::Result;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use firestore::{FirestoreDb, FirestoreDbOptions};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::FirebaseUser;

impl FromRequestParts<AppState> for FirestoreDb
where
    AppState: FromRef<AppState>,
    AppState: Send + Sync,
{
    type Rejection = DbNotFoundResponse;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let db = FirestoreDb::from_ref(&state.db);
        Ok(db)
    }
}

pub struct DbNotFoundResponse {
    msg: String,
}

impl IntoResponse for DbNotFoundResponse {
    fn into_response(self) -> Response {
        (StatusCode::NOT_FOUND, self.msg).into_response()
    }
}

pub async fn new_db() -> Result<FirestoreDb> {
    let options = FirestoreDbOptions {
        google_project_id: "r-j-magenta-carrot-42069".to_string(),
        database_id: "list-site".to_string(),
        max_retries: 3,
        firebase_api_url: None,
    };
    match FirestoreDb::with_options(options).await {
        Ok(db) => Ok(db),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub items: Vec<String>,
}

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

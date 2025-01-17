use std::sync::Arc;

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

use super::FirebaseUser;

impl FromRequestParts<AppState> for Arc<FirestoreDb>
where
    AppState: FromRef<AppState>,
    AppState: Send + Sync,
{
    type Rejection = DbNotFoundResponse;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(&state);
        let db = state.db;
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

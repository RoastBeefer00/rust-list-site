use crate::AppState;
use anyhow::anyhow;
use anyhow::Result;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use firestore::FirestoreDb;

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
    match FirestoreDb::new("r-j-magenta-carrot-42069").await {
        Ok(db) => Ok(db),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

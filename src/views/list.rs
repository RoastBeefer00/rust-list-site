use std::sync::Arc;

use crate::db::User;
use anyhow::{Context, Result};
use askama_axum::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
// use firebase_auth::FirebaseUser;
use crate::db::FirebaseUser;
use firestore::FirestoreDb;

use super::{list_item::ListItem, ListPreview};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateListForm {
    pub name: String,
}

#[derive(Deserialize)]
struct UpdateListForm {
    text: String,
}

#[derive(Deserialize)]
pub(crate) struct ShareListForm {
    user_id: Uuid,
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

    // CRUD opeations
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

    // View handlers
    pub async fn get_view(State(db): State<FirestoreDb>, Path(id): Path<Uuid>) -> Response {
        match Self::get(id, &db).await {
            Ok(list) => list.into_response(),
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
            owner: user.clone().email.unwrap_or(user.clone().user_id),
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
        ListPreview::from(list).into_response()
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
            Ok(_) => list.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    pub async fn share(
        State(db): State<FirestoreDb>,
        Path(id): Path<Uuid>,
        Form(share_form): Form<ShareListForm>,
    ) -> impl IntoResponse {
        let user_id = share_form.user_id;
        let user = User::from(user_id);
        if let Some(mut db_user) = user.get(&db).await.unwrap() {
            if let Err(e) = db_user.grant_access(id, &db).await {
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            } else {
                return (StatusCode::OK).into_response();
            }
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }
}

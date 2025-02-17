use anyhow::{Context, Result};
use firestore::{struct_path::path, FirestoreDb, FirestoreQueryDirection};
use futures::stream::BoxStream;
use tokio_stream::StreamExt;

// use firebase_auth::FirebaseUser;
use crate::db::FirebaseUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::views::{List, ListGroup};

// A user from the site
// Contains the lists the user has access to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub lists: Vec<Uuid>,
}

impl From<FirebaseUser> for User {
    fn from(user: FirebaseUser) -> Self {
        User {
            id: user.user_id,
            name: user.name,
            email: user.email,
            lists: vec![],
        }
    }
}

impl From<&FirebaseUser> for User {
    fn from(user: &FirebaseUser) -> Self {
        User {
            id: user.user_id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            lists: vec![],
        }
    }
}

impl From<Uuid> for User {
    fn from(id: Uuid) -> Self {
        User {
            id: id.to_string(),
            name: None,
            email: None,
            lists: vec![],
        }
    }
}

impl User {
    pub async fn get(&self, db: &FirestoreDb) -> Result<Option<User>> {
        db.fluent()
            .select()
            .by_id_in("users")
            .obj()
            .one(&self.id)
            .await
            .context("Failed to get user")
    }

    pub async fn get_all(db: &FirestoreDb) -> Result<Vec<User>> {
        let users_stream: BoxStream<User> = db
            .fluent()
            .list()
            .from("users")
            .page_size(100)
            .order_by([(path!(User::id), FirestoreQueryDirection::Descending)])
            .obj()
            .stream_all()
            .await?;

        let users: Vec<User> = users_stream.collect().await;

        Ok(users)
    }

    pub async fn create(&self, db: &FirestoreDb) -> Result<User> {
        db.fluent()
            .insert()
            .into("users")
            .document_id(&self.id)
            .object(self)
            .execute::<User>()
            .await
            .context("Failed to create user")
    }

    pub async fn update(&self, db: &FirestoreDb) -> Result<User> {
        db.fluent()
            .update()
            .in_col("users")
            .document_id(&self.id)
            .object(self)
            .execute::<User>()
            .await
            .context("Failed to update user")
    }

    pub async fn get_all_list_groups(&self, db: &FirestoreDb) -> Result<Vec<ListGroup>> {
        let mut lists = Vec::new();
        let mut tasks = Vec::new();
        for id in self.lists.clone() {
            let db = db.clone();
            tasks.push(tokio::spawn(async move { List::get(id, &db).await }));
        }
        for task in tasks {
            let list = task.await??;
            lists.push(list);
        }

        let mut users: Vec<String> = Vec::new();
        let mut groups: Vec<ListGroup> = Vec::new();
        for list in lists.clone() {
            if !users.contains(&list.owner) {
                users.push(list.owner);
            }
        }

        for user in users {
            let user_lists = lists
                .clone()
                .into_iter()
                .filter(|list| list.owner == user)
                .collect();
            groups.push(ListGroup {
                owner: user,
                lists: user_lists,
            })
        }

        Ok(groups)
    }

    pub async fn grant_access(&mut self, id: Uuid, db: &FirestoreDb) -> Result<User> {
        self.lists.push(id);
        self.update(db).await
    }

    pub async fn remove_access(&mut self, id: Uuid, db: &FirestoreDb) -> Result<User> {
        self.lists.retain(|&list_id| list_id != id);
        self.update(db).await
    }
}

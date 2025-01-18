use anyhow::{Context, Result};
use firestore::FirestoreDb;

// use firebase_auth::FirebaseUser;
use crate::db::FirebaseUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::views::List;

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

impl User {
    pub async fn get(user: &FirebaseUser, db: &FirestoreDb) -> Result<Option<User>> {
        db.fluent()
            .select()
            .by_id_in("users")
            .obj()
            .one(&user.user_id)
            .await
            .context("Failed to get user")
    }

    pub async fn create(user: &FirebaseUser, db: &FirestoreDb) -> Result<User> {
        let user = User::from(user.clone());
        db.fluent()
            .insert()
            .into("users")
            .document_id(&user.id)
            .object(&user)
            .execute::<User>()
            .await
            .context("Failed to create user")
    }

    pub async fn update(user: &User, db: &FirestoreDb) -> Result<User> {
        db.fluent()
            .update()
            .in_col("users")
            .document_id(&user.id)
            .object(user)
            .execute::<User>()
            .await
            .context("Failed to update user")
    }

    pub async fn get_all_lists(&self, db: &FirestoreDb) -> Result<Vec<List>> {
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

        Ok(lists)
    }
}

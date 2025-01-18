use anyhow::Context;
use anyhow::Result;
use firestore::{FirestoreDb, FirestoreDbOptions};

pub async fn new_db() -> Result<FirestoreDb> {
    let options = FirestoreDbOptions {
        google_project_id: "r-j-magenta-carrot-42069".to_string(),
        database_id: "list-site".to_string(),
        max_retries: 3,
        firebase_api_url: None,
    };
    FirestoreDb::with_options(options)
        .await
        .context("Failed to create FirestoreDb")
}

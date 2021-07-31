use deadpool_postgres::Pool;
use eyre::{eyre, Result};
use std::sync::Arc;
use tracing::instrument;

#[instrument(skip(db_pool))]
pub async fn create_user(db_pool: Arc<Pool>, name: &String) -> Result<i64> {
    let db_pool = db_pool.clone();
    let mut client = db_pool.get().await?;

    let tx = client.transaction().await?;

    let result = tx
        .query(include_str!("sql/create_user.sql"), &[name])
        .await?;

    let id = result
        .iter()
        .map(|row| {
            let id: i64 = row.get(0);
            id
        })
        .next()
        .ok_or(eyre!("Unable to create user"))?;

    tx.commit().await?;

    Ok(id)
}

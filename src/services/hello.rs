use deadpool_postgres::Pool;
use eyre::Result;
use std::sync::Arc;

pub async fn hello(db_pool: Arc<Pool>) -> Result<String> {
    let db_pool = db_pool.clone();
    let client = db_pool.get().await?;
    let result = client.query("SELECT version()", &[]).await?;

    let version: &str = result[0].get(0);

    Ok(format!("Hello {}", version))
}

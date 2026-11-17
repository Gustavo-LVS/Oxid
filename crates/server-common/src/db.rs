use sqlx::{PgPool, postgres::PgPoolOptions};

/// Abre o pool de conexões e aplica as migrações em `migrations/`.
pub async fn connect_and_migrate(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;
    sqlx::migrate!("../../migrations").run(&pool).await?;
    Ok(pool)
}

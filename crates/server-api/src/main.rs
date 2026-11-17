use oxid_server_common::{config::Config, db, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init();
    let config = Config::from_env()?;
    let pool = db::connect_and_migrate(&config.database_url).await?;
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    tracing::info!(addr = %config.bind_addr, "oxid-server-api ouvindo");
    axum::serve(listener, oxid_server_api::app(pool, &config)).await?;
    Ok(())
}

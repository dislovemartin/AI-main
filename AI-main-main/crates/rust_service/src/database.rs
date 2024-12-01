use anyhow::{Result, anyhow};
use tokio_postgres::NoTls;

pub async fn connect_database() -> Result<tokio_postgres::Client> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=secret dbname=mydb", NoTls)
            .await
            .map_err(|e| anyhow!("Database connection failed: {}", e))?;

    // Spawn the connection to run in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

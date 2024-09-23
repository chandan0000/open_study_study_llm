use sea_orm::{ConnectOptions, Database, DatabaseConnection};
// use std::env;
use anyhow::{self, Error};
use log::{self, info};
use std::time::Duration;
// use std::env;

pub async fn establish_connection(
    database_url:&str
) -> Result<DatabaseConnection, Error> {
 
    // Construct the database URL
    // "DATABASE_URL");
    // let database_url = dotenv!("DATABASE_URL");

    info!("database url {database_url}");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await;
    match db {
        Ok(connection) => Ok(connection),
        Err(e) => {
            eprintln!("Failed to connect to the database: {:?}", e);
            Err(e.into())
        }
    }
}

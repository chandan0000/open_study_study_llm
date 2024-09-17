use dotenvy_macro::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
// use std::env;
use anyhow::{self, Error};
use log;
use std::env;
use std::time::Duration;
// use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, Error> {
    // let database_url = env!("DATABASE_URL");
    // let database_url =  dotenv!("DATABASE_URL");
    // Read environment variables
    let db_host = dotenv!("DB_HOST");
    let db_user = dotenv!("DB_USER");
    let db_password = dotenv!("DB_PASSWORD");
    let db_name = dotenv!("DB_NAME");
    let db_port = dotenv!("DB_PORT");

    // Construct the database URL
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    //   let database_url = "postgresql://postgres:12345@localhost:5435/jh";
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

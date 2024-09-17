mod db;
mod routes;

use db::connection::establish_connection;
use routes::create_routes;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db = establish_connection().await?;
    tracing::subscriber::set_global_default(FmtSubscriber::default());

    // Initialize database connection

    // Run all pending migrations
    
    // Migrator::up(&db, None).await.unwrap();

    
    let app = create_routes(db);
    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
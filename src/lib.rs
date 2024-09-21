mod app_state;
mod db;
mod mail;
mod middleware;
mod routes;
mod utilities;

use dotenvy_macro::dotenv;

use axum::{routing::get, Router};
use db::connection::establish_connection;
use mail::{mails::send_welcome_email, sendmail::send_email};
use migration::MigratorTrait;
use routes::create_routes;
use tracing::{info, level_filters::LevelFilter};
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db = establish_connection(dotenv!("DATABASE_URL")).await?;
    // let _ = tracing::subscriber::set_global_default(FmtSubscriber::default());
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Initialize database connection
    // Run all pending migrations
    // Migrator::up(&db, None).await.unwrap();

    migration::Migrator::up(&db, None).await?;

    let app = Router::new()
        .route("/", get(|| async { "<h1> i am alive </h1>".to_string() }))
        .nest("/api", create_routes(db));

    let send_welcome_email_result = send_welcome_email("kumarchandan41u@gmail.com", "kumarchanan").await;

    if let Err(e) = send_welcome_email_result {
        eprintln!("Failed to send welcome email: {}", e);
    }

    // let host_url = dotenv!("HOST_URL");
    // let host_port = dotenv!("HOST_PORT");

    info!("Starting server at http://0.0.0.0:8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

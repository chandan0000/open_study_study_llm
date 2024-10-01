mod api_middleware;
mod app_state;
mod db;
mod graphql;
mod handler;
mod mail;
mod query_root;
mod routes;
mod utilities;
use async_graphql::http::GraphiQLSource;
use dotenvy_macro::dotenv;

use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use db::connection::establish_connection;
use migration::MigratorTrait;
use routes::create_routes;
use tracing::{info, level_filters::LevelFilter};

use async_graphql_axum::{GraphQL, GraphQLSubscription};

async fn graphql_playground() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db = establish_connection(dotenv!("DATABASE_URL")).await?;
    // let _ = tracing::subscriber::set_global_default(FmtSubscriber::default());
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Initialize database connection
    // Run all pending migrations
    // Migrator::up(&db, None).await.unwrap();

    // migration::Migrator::up(&db, None).await?;
    // let schema = crate::query_root::schema(db.clone(), None, None)?;

    let app = Router::new()
        .route(
            "/health",
            get(|| async { "<h1> i am alive </h1>".to_string() }),
        )
        // .route(
        //     "/",
        //     get(graphql_playground).post_service(GraphQL::new(schema.clone())),
        // )
        // .route_service("/ws", GraphQLSubscription::new(schema))
        .nest("/api", create_routes(db));

    // let send_welcome_email_result = send_welcome_email("kumarchandandbg1@gmail.com", "chandan").await;

    // if let Err(e) = send_welcome_email_result {
    //     eprintln!("Failed to send welcome email: {}", e);
    // }

    info!("Starting server at http://127.0.0.0:8080");
    // let res = mail::mails::send_welcome_email("kumarchandan41u@gmail.com", "huma").await;

    // match res {
    //     Ok(_) => {
    //         info!("Email sent successfully");
    //     }
    //     Err(e) => {
    //         info!("Error sending email: {:?}", e);
    //     }
    // }



    let listener = tokio::net::TcpListener::bind("127.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

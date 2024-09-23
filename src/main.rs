use dotenvy::dotenv;
use openstudystudyllm::run;
#[tokio::main]
async fn main() {
    dotenv().ok();

    run().await.unwrap();
}


use std::result;
use  open_study_study_llm::run;
use dotenvy::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // cli::run_cli(migration::Migrator).await;
    // let result = send_otp("+917976650377", ).await;
    // match result {
    //     Ok(res) => {
    //         println!("otp send")
    //     }
    //     Err(e) => {
    //         println!("error {}", e)
    //     }
    // }
    run().await.unwrap();
}

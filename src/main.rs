mod commands;
mod utils;
mod framework;


#[tokio::main]
async fn main(){
    // Load .env file if it exists. Falls back to loading the variables from the actual environment
    dotenv::dotenv().ok();

    framework::build().run_autosharded().await.expect("Error");
}

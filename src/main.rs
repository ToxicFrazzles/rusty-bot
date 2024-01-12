use poise::serenity_prelude::{GatewayIntents, ClientBuilder};
use songbird::SerenityInit;
use utils::get_token;

mod commands;
mod utils;
mod framework;
mod logic;
mod error;

mod tests;

#[tokio::main]
async fn main(){
    // Load .env file if it exists. Falls back to loading the variables from the actual environment
    dotenv::dotenv().ok();

    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT 
        | GatewayIntents::GUILD_VOICE_STATES 
        | GatewayIntents::GUILD_MEMBERS;
    // .client_settings(|c| c.register_songbird())
    let framework = framework::build().await;
    let mut client = ClientBuilder::new(get_token(), intents)
        .framework(framework).register_songbird().await.expect("Error creating client");
 
    client.start_autosharded().await.expect("Client Error");

    // framework::build().run_autosharded().await.expect("Error");
}

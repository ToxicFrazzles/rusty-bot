use std::env;

use poise::serenity_prelude::{GatewayIntents, ClientBuilder};
use songbird::SerenityInit;
use utils::get_token;

mod commands;
mod utils;
mod framework;
mod logic;
mod error;

mod checks;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main(){
    // Load .env file if it exists. Falls back to loading the variables from the actual environment
    dotenv::dotenv().ok();
    
    let logging_level = match env::var("LOG_LEVEL"){
        Ok(v) => {
            match v.to_lowercase().as_str(){
                "debug" => tracing::Level::DEBUG,
                "info" => tracing::Level::INFO,
                "warn" => tracing::Level::WARN,
                "warning" => tracing::Level::WARN,
                "error" => tracing::Level::ERROR,
                _ => tracing::Level::WARN
            }
        },
        _ => tracing::Level::WARN
    };

    tracing_subscriber::fmt()
        .with_max_level(logging_level)
        .with_test_writer()
        .init();

    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT 
        | GatewayIntents::GUILD_VOICE_STATES 
        | GatewayIntents::GUILD_MEMBERS;
    let framework = framework::build().await;
    let mut client = ClientBuilder::new(get_token(), intents)
        .framework(framework).register_songbird().await.expect("Error creating client");

    tokio::spawn(async move{
        // Start the bot
        client.start_autosharded().await.expect("Client Error");
    });
    let _signal_err = tokio::signal::ctrl_c().await;
}

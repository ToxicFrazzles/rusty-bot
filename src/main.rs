mod commands;
mod utils;

use serenity::{
    framework::{
        StandardFramework, 
        standard::macros::group
    }, 
    model::gateway::{
            GatewayIntents, Ready, Activity
    }, 
    client::{
        Context, EventHandler
    }, 
    Client, async_trait
};
use songbird::SerenityInit;

use crate::{
    commands::{
        audio::{
            play::*,
            skip::*,
        },
        ping::*,
        about::*,
    }, 
    utils::{
        get_status, get_shard_count
    }
};
use crate::utils::{get_prefix, get_token};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard{
            println!("{} is connected on shard {}/{}!", ready.user.name, shard[0]+1, shard[1]);
            let status = get_status();
            ctx.set_activity(Activity::playing(status)).await;
        }
    }
}

#[group]
#[commands(
    about,  ping, 
    
    // Audio commands
    play,   skip
)]
struct General;


#[tokio::main]
async fn main(){
    println!("{:?}", GENERAL_GROUP);
    // Load .env file if it exists. Falls back to loading the variables from the actual environment
    dotenv::dotenv().ok();

    // Gather important configs from the environment erroring if they don't exist.
    let token = get_token();
    let prefix = get_prefix();

    let framework = StandardFramework::new().group(&GENERAL_GROUP)
        .configure(|c| c.prefix(prefix));
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        .event_handler(Handler)
        .await
        .expect("Error creating client!!!");

    let manager = client.shard_manager.clone();


    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register Ctrl-C handler");
        println!("Received Ctrl-C, Shutting down...");
        manager.lock().await.shutdown_all().await;
    });

    let shard_count = get_shard_count();
    if shard_count > 0 {
        let _ = client.start_shards(shard_count)
        .await
        .map_err(|why| println!("Client ended: {:?}", why));   
    }else{
        let _ = client.start_autosharded()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));   
    }
}

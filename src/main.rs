mod commands;
mod utils;

use serenity::{framework::{StandardFramework, standard::{macros::{group, command}, CommandResult}}, model::{channel::Message, gateway::{GatewayIntents, Ready, Activity}}, client::{Context, EventHandler}, Client, async_trait};
use songbird::SerenityInit;

use crate::{commands::audio::play::*, utils::{get_status, get_shard_count}};
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
#[commands(ping, play)]
struct General;


#[tokio::main]
async fn main(){
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

    // let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        let _ = client.start_shards(get_shard_count())
            .await
            .map_err(|why| println!("Client ended: {:?}", why));   
    });

    let _signal_err = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, Shutting down...");
}


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult{
    msg.channel_id.say(&ctx, "Pong!").await.err();
    Ok(())
}
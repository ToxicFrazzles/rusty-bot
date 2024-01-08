use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::utils::send_message;

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult{
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.expect("Songbird Voice Client failed to initialise").clone();
    if manager.get(guild_id).is_none(){
        send_message(msg, ctx, "Not playing audio in this server").await;
        return Ok(());
    }

    if let Some(handler_lock) = manager.get(guild_id){   
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if queue.is_empty() {
            send_message(msg, ctx, "Nothing in the queue to skip").await;
        }else{
            match queue.skip() {
                Ok(_) => send_message(msg, ctx, "Skipped...").await,
                Err(why) => send_message(msg, ctx, format!("Error skipping: {:?}", why)).await
            };
        }   
    }

    Ok(())
}
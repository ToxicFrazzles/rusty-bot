use std::env;

use serenity::{framework::standard::{macros::command, Args, CommandResult}, client::Context, model::channel::Message};

use crate::utils::send_message;


#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    let prefix = env::var("PREFIX").expect("Expected PREFIX in environment");

    let url = match args.clone().single::<String>(){
        Ok(url) => url.clone(),
        Err(_) => {
            send_message(msg, ctx, format!("Use the command like {prefix}play <url or song name>")).await;
            return Ok(());
        }
    };

    let search_terms = args.clone();
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.expect("Songbird Voice Client failed to initialise").clone();

    if manager.get(guild_id).is_none(){
        let channel_id = guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id);
        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                send_message(msg, ctx, "Join a voice channel first!").await;
                return Ok(())
            }
        };
        let manager = songbird::get(ctx).await.expect("Songbird voice client failed to initialise").clone();
        let (_, success) = manager.join(guild_id, connect_to).await;
        if let Err(_channel) = success {
            send_message(msg, ctx, format!("Error joining channel! Perhaps I don't have permissions to do that :(")).await;
        }
    }
    if let Some(handler_lock) = manager.get(guild_id){
        if !url.starts_with("http") {
            // Not a direct URL, search for the message contents on youtube
            let mut handler = handler_lock.lock().await;
            if let Some(src) = match songbird::input::ytdl_search(search_terms.message()).await {
                Ok(src) => Some(src),
                Err(_) => {
                    send_message(msg, ctx, "Failed to find the source requested").await;
                    None
                }
            }{
                let track = handler.enqueue_source(src);
                let metadata = track.metadata();

                send_message(msg, ctx, format!("Added {} to queue. Position: {}", metadata.title.clone().unwrap(), handler.queue().len())).await;
            }
        }
    }


    Ok(())
}
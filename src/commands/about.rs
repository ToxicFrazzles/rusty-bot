use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::utils::{send_message, get_prefix};

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult{
    let prefix = get_prefix();
    send_message(msg, ctx, format!("I'm rusty-bot a Discord bot written in Rust with the Serenity and Songbird libraries.
My command prefix is `{prefix}`")).await;
    Ok(())
}
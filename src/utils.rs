use std::env;

use serenity::{model::channel::Message, client::Context};

pub fn get_prefix() -> String {
    env::var("PREFIX").expect("Expected PREFIX in environment")
}

pub fn get_token() -> String {
    env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment")
}

pub fn get_status() -> String {
    let prefix = get_prefix();
    env::var("DISCORD_STATUS").or::<String>(Ok(format!("{prefix}help"))).unwrap()
}

pub fn get_shard_count() -> u64 {
    match env::var("SHARDS_COUNT"){
        Ok(str_count) => {str_count.parse::<u64>().expect(format!("Could not parse {str_count} as a 64-bit integer").as_str())},
        Err(_) => {5}
    }
}


#[inline]
pub async fn send_message<D: ToString>(msg: &Message, ctx: &Context, msg_content: D){
    _send_message(msg, ctx, msg_content.to_string()).await;
}

async fn _send_message(msg: &Message, ctx: &Context, msg_content: String){
    msg.channel_id.send_message(&ctx, |m| {
        m.content(msg_content)
    }).await.expect(format!("Failed to send message to {:?}", msg.channel_id).as_str());
}
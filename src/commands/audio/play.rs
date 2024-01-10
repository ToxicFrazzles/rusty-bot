use songbird::input::{Restartable, Input};
use url::Url;

use crate::commands::{Context, Error};
use super::utils::join_channel;


#[poise::command(prefix_command, broadcast_typing, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "What to play"]
    #[rest]
    what: String
) -> Result<(), Error>{
    let (_guild_id, _channel_id, conn, _manager) = join_channel(&ctx).await?;

    let src: Input = if let Ok(url) = Url::parse(&what) {
        Restartable::ytdl(url, true).await?.into()
    }else{
        Restartable::ytdl_search(what, true).await?.into()
    };

    let metadata = src.metadata.clone();
    let _handle = conn.lock().await.enqueue_source(src);

    ctx.send(|r| {
        r.content(format!("Queueing audio: {}", metadata.title.or(Some("No Title Found".to_string())).unwrap()))
    }).await?;

    Ok(())
}
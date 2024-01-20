use songbird::input::Input;
use songbird::input::YoutubeDl;
use songbird::tracks::Track;
use url::Url;

use crate::commands::{Context, Error};
use super::utils::join_channel;

/// Play something in the voice chat
/// 
/// Play something either giving the search terms ot the direct URL
/// !play rickroll
/// !play <https://youtu.be/dQw4w9WgXcQ?si=PGAeVqQDRDPFTkI->
#[poise::command(prefix_command, broadcast_typing, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "What to play"]
    #[rest]
    mut what: String
) -> Result<(), Error>{
    let (_guild_id, _channel_id, conn, _manager) = join_channel(&ctx).await?;
    let req_client = ctx.data().reqwest.clone();
    what = what.trim_start_matches("<").trim_end_matches(">").to_string();

    let mut src: Input = if let Ok(url) = Url::parse(&what) {
        YoutubeDl::new(req_client, url.to_string()).into()
    }else{
        YoutubeDl::new_search(req_client, what).into()
    };

    let metadata = src.aux_metadata().await.unwrap();
    if metadata.duration == None{
        ctx.say("The track requested is either a livestream or no duration could be found").await?;
        return Ok(());
    }
    let _handle = conn.lock().await.enqueue_input(src).await;

    let title = metadata.title.or(Some("No Title Found".to_string())).unwrap();

    ctx.say(format!("Queueing audio: {title}")).await?;
    
    Ok(())
}
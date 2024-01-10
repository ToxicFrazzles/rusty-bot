use crate::commands::{Result, Context, audio::utils::get_conn};

/// Skip the currently playing audio track
#[poise::command(prefix_command, guild_only)]
pub async fn skip(
    ctx: Context<'_>
) -> Result<()>{
    let conn = get_conn(&ctx).await?;
    if !conn.lock().await.queue().is_empty() {
        ctx.say("Skipping track...").await?;
        let _ = conn.lock().await.queue().skip();
    }else{
        ctx.say("Nothing in the queue to skip").await?;
    }
    Ok(())
}
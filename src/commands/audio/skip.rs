use crate::commands::{Result, Context, audio::utils::get_conn};

/// Skip the currently playing audio track
#[poise::command(prefix_command, guild_only)]
pub async fn skip(
    ctx: Context<'_>
) -> Result<()>{
    let conn = get_conn(&ctx).await?;
    let _ = conn.lock().await.queue().skip();

    Ok(())
}
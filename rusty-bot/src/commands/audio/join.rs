use crate::commands::{Context, Result, audio::utils::join_channel};

/// Leave the voice channel
#[poise::command(prefix_command, guild_only, hide_in_help)]
pub async fn join(ctx: Context<'_>) -> Result<()>{
    let (_guild_id, _channel_id, _conn, _manager) = join_channel(&ctx).await?;
    Ok(())
}
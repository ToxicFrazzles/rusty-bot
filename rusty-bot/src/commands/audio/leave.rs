use crate::commands::{Context, Result, audio::utils::leave_server};

/// Leave the voice channel
#[poise::command(prefix_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<()>{
    leave_server(&ctx).await?;
    ctx.say("Bye 👋").await?;
    Ok(())
}
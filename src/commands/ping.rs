use crate::commands::{Context, Error};

/// Ping pong test
#[poise::command(prefix_command)]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error>{
    ctx.say("Pong!").await?;
    Ok(())
}
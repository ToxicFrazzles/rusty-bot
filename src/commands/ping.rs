use crate::commands::{Context, Error};
use tracing::{event, Level};

/// Ping pong test
#[poise::command(prefix_command)]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error>{
    event!(Level::DEBUG, "Ping command");
    ctx.say("Pong!").await?;
    Ok(())
}
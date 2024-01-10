use crate::commands::{Context, Error};

#[poise::command(prefix_command)]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error>{
    ctx.say("Pong!").await?;
    // println!("Ping! Pong!");
    Ok(())
}
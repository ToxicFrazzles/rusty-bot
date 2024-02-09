use database::Guild;

use crate::commands::{Context, Result};


const MAX_TIMEOUT_HOURS: i64 = 8;
const MAX_TIMEOUT_MINUTES: i64 = MAX_TIMEOUT_HOURS*60;


#[poise::command(prefix_command, rename="vc_timeout")]
pub async fn set(
    ctx: Context<'_>,
    timeout: i64
) -> Result<()>{
    let db = &ctx.data().db;
    let guild_id = ctx.guild_id().unwrap().to_string();

    if timeout > 60*4{
        ctx.say(format!("Maximum timeout is {} hours ({} minutes)", MAX_TIMEOUT_HOURS, MAX_TIMEOUT_MINUTES)).await?;
        return Ok(());
    }
    if timeout < 0{
        ctx.say("Silly. You can't set the voice chat timeout to a negative number...").await?;
        return Ok(())
    }
    Guild::set_vc_timeout(db, &guild_id, timeout).await?;
    ctx.say(format!("Set the voice chat timeout to {timeout} minutes")).await?;

    Ok(())
}


#[poise::command(prefix_command, rename="vc_timeout")]
pub async fn get(
    ctx: Context<'_>
) -> Result<()>{
    let db = &ctx.data().db;
    let guild_id = ctx.guild_id().unwrap().to_string();
    let guild = Guild::find_by_snowflake_or_default(db, &guild_id).await;

    ctx.say(format!("Voice chat timeout is currently set to {} minutes", guild.vc_timeout)).await?;
    
    Ok(())
}
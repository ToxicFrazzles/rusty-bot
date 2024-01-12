use crate::commands::{Context, Result};
use crate::logic::blacklist;


/// List all users which are blacklisted from using the bot in this server
#[poise::command(prefix_command, guild_only, subcommands("global", "guild_add", "guild_remove"))]
pub async fn blacklist(
    ctx: Context<'_>
) -> Result<()>{
    todo!("Implement listing blacklisted users");
    Ok(())
}

/// Add user to the guild blacklist to prevent them from using the bot in this server
#[poise::command(prefix_command, rename="add")]
pub async fn guild_add(
    ctx: Context<'_>,
    #[description = "User to add to the guild blacklist"]
    who: String
)->Result<()>{
    let db = &ctx.data().db;
    let guild_id = ctx.guild_id().expect("No guild ID").to_string();
    blacklist::guild_add(db, who, guild_id).await?;
    ctx.say("Added user to the guild blacklist").await?;
    Ok(())
}

/// Remove user from the guild blacklist to reallow them to use the bot in this server
#[poise::command(prefix_command, rename="remove")]
pub async fn guild_remove(
    ctx: Context<'_>,
    #[description = "User to remove from the guild blacklist"]
    who: String
)->Result<()>{
    let db = &ctx.data().db;
    let guild_id = ctx.guild_id().expect("No guild ID").to_string();
    blacklist::guild_remove(db, who, guild_id).await?;
    ctx.say("Removed user from the guild blacklist").await?;
    Ok(())
}

/// List all users which are blacklisted from using the bot in any guild or even DMs
#[poise::command(prefix_command, owners_only, subcommands("global_add", "global_remove"))]
pub async fn global(
    ctx: Context<'_>
) -> Result<()>{
    todo!("Implement listing blacklisted users");
    Ok(())
}

/// Add a user to the global blacklist to prevent them from using the bot
#[poise::command(prefix_command, rename="add")]
pub async fn global_add(
    ctx: Context<'_>,
    #[description = "User to add to the global blacklist"]
    who: String
)->Result<()>{
    let db = &ctx.data().db;
    blacklist::global_add(db, who).await?;
    ctx.say("Added user to the global blacklist").await?;
    Ok(())
}

/// Remove user from the global blacklist to reallow them to use the bot
#[poise::command(prefix_command, rename="remove")]
pub async fn global_remove(
    ctx: Context<'_>,
    #[description = "User to remove from the global blacklist"]
    who: String
)->Result<()>{
    let db = &ctx.data().db;
    blacklist::global_remove(db, who).await?;
    ctx.say(format!("Removed user from the global blacklist")).await?;
    Ok(())
}

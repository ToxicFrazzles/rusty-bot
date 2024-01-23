use poise::CreateReply;
use serenity::builder::CreateAllowedMentions;

use crate::commands::{Context, Result};
use crate::logic::blacklist;


/// List all users which are blacklisted from using the bot in this server
#[poise::command(
    prefix_command, guild_only, 
    required_permissions = "KICK_MEMBERS | BAN_MEMBERS | MODERATE_MEMBERS",
    subcommands("global", "guild_add", "guild_remove"))]
pub async fn blacklist(
    ctx: Context<'_>
) -> Result<()>{
    let db = &ctx.data().db;
    let guild_id = ctx.guild_id().unwrap().to_string();
    let blacklisted = blacklist::guild_list(db, guild_id).await?;
    if blacklisted.len() == 0{
        ctx.say("No users in this guild are blacklisted").await?;
        return Ok(());
    }

    let mut the_list = String::new();
    blacklisted.iter().for_each(|i|{
        the_list = format!("{}<@{}>\n", the_list.clone(), i);
    });
    let the_list = the_list.trim();
    // println!("{}", the_list);

    ctx.send(CreateReply::default()
            .allowed_mentions(CreateAllowedMentions::new().empty_users())
            .content(the_list)
        ).await?;
    
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
    blacklist::guild_set(db, who, guild_id, true).await?;
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
    blacklist::guild_set(db, who, guild_id, false).await?;
    ctx.say("Removed user from the guild blacklist").await?;
    Ok(())
}

/// List all users which are blacklisted from using the bot in any guild or even DMs
#[poise::command(
    prefix_command, owners_only, hide_in_help,
    subcommands("global_add", "global_remove"))]
pub async fn global(
    ctx: Context<'_>
) -> Result<()>{
    let db = &ctx.data().db;
    let blacklisted = blacklist::global_list(db).await?;
    if blacklisted.len() == 0{
        ctx.say("No users are in the global blacklist").await?;
        return Ok(());
    }
    let mut the_list = String::new();
    blacklisted.iter().for_each(|i|{
        the_list = format!("{}<@{}>\n", the_list.clone(), i);
    });
    let the_list = the_list.trim();
    // println!("{}", the_list);

    ctx.send(CreateReply::default()
            .allowed_mentions(CreateAllowedMentions::new().empty_users())
            .content(the_list)
        ).await?;
    
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
    blacklist::global_set(db, who, true).await?;
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
    blacklist::global_set(db, who, false).await?;
    ctx.say(format!("Removed user from the global blacklist")).await?;
    Ok(())
}

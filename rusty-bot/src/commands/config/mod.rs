// Config get and config set commands
use poise::CreateReply;
use serenity::builder::CreateAllowedMentions;

use crate::commands::{Context, Result};

mod vc_timeout;


#[poise::command(
    prefix_command, guild_only, hide_in_help,
    required_permissions = "ADMINISTRATOR | MANAGE_GUILD",
    subcommand_required,
    subcommands("set", "get"))]
pub async fn config(
    ctx: Context<'_>
) -> Result<()>{
    Ok(())
}


#[poise::command(
    prefix_command, subcommand_required,
    subcommands("vc_timeout::set")
)]
pub async fn set(
    ctx: Context<'_>
) -> Result<()>{
    todo!("Implement config set command");
    Ok(())
}


#[poise::command(
    prefix_command, subcommand_required,
    subcommands("vc_timeout::get")
)]
pub async fn get(
    ctx: Context<'_>
) -> Result<()>{
    todo!("Implement config get command");
    Ok(())
}
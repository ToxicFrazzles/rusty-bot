use poise::serenity_prelude::*;
use poise::CreateReply;

use crate::commands::{Context, Error};


fn get_invite_url(ctx: Context<'_>)->String{
    let bot_id = ctx.cache().current_user().id.to_string();
    let perms = "412320385024";
    format!("https://discord.com/oauth2/authorize?client_id={bot_id}&scope=bot&permissions={perms}")
}


#[poise::command(prefix_command)]
pub async fn info(
    ctx: Context<'_>
) -> Result<(), Error>{
    let invite_url = get_invite_url(ctx);
    
    ctx.send(CreateReply::default()
        .embed(CreateEmbed::default()
            .title("Bot Info")
            .description(format!("I am very much a work in progress. Do not expect stability or reliability from me. My creator <@169536101357191168> is a moron.\n\
            That said, you can add me to your own server by clicking [this link]({invite_url})"))
        )
    ).await?;
    Ok(())
}
use std::sync::Arc;

use poise::serenity_prelude::{GuildId, ChannelId};
use songbird::{Call, Songbird};
use tokio::sync::Mutex;

use crate::commands::{Context, Result};
use crate::error::{NotInGuildError, NotInVoiceChannelError, NoVoiceChannelIdError, NoSongbirdError};

pub async fn get_conn(ctx: &Context<'_>) -> Result<Arc<Mutex<Call>>>{
    let guild_id = ctx.guild().ok_or(NotInGuildError)?.id;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;
    let conn = manager.get(guild_id).ok_or(NotInVoiceChannelError)?;

    Ok(conn)
}


pub async fn join_channel(ctx: &Context<'_>) -> Result<(GuildId, ChannelId, Arc<Mutex<Call>>, Arc<Songbird>)>{
    let guild_id: GuildId;
    let channel_id: ChannelId;
    {
        let guild = ctx.guild().ok_or(NotInGuildError)?;
        guild_id = guild.id;

        let channel = guild
            .voice_states
            .get(&ctx.author().id)
            .ok_or(NotInVoiceChannelError)?;
        channel_id = channel.channel_id.ok_or(NoVoiceChannelIdError)?;
    }


    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;

    let conn = match manager.join(guild_id, channel_id).await {
        Ok(c) => Ok(c),
        Err(err) => {
            if err.should_leave_server() {
                manager.get(guild_id).unwrap().lock().await.leave().await?;
            }
            Err(err)
        }
    }?;

    Ok((guild_id, channel_id, conn, manager))
}


pub async fn leave_server(ctx: &Context<'_>) -> Result<()>{
    let guild_id = ctx.guild().ok_or(NotInGuildError)?.id;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;
    let conn = manager.get(guild_id).ok_or(NotInVoiceChannelError)?;
    conn.lock().await.leave().await?;
    Ok(())
}

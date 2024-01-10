use std::sync::Arc;

use poise::serenity_prelude::{GuildId, ChannelId};
use songbird::{Call, Songbird};
use tokio::sync::Mutex;

use crate::commands::{Context, Result, error::{NotInGuildError, NotInVoiceChannelError, NoVoiceChannelIdError, NoSongbirdError}};


pub async fn get_conn(ctx: &Context<'_>) -> Result<Arc<Mutex<Call>>>{
    let guild = ctx.guild().ok_or(NotInGuildError)?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;
    let conn = manager.get(guild.id).ok_or(NotInVoiceChannelError)?;

    Ok(conn)
}


pub async fn join_channel(ctx: &Context<'_>) -> Result<(GuildId, ChannelId, Arc<Mutex<Call>>, Arc<Songbird>)>{
    let guild = ctx.guild().ok_or(NotInGuildError)?;

    let channel = guild
        .voice_states
        .get(&ctx.author().id)
        .ok_or(NotInVoiceChannelError)?;

    let channel_id = channel.channel_id.ok_or(NoVoiceChannelIdError)?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;

    let conn = {
        let (conn, result) = manager.join(guild.id, channel_id).await;
        if let Err(err) = result {
            if err.should_leave_server() {
                conn.lock().await.leave().await?;
            }
            return Err(err.into());
        }
        conn
    };

    Ok((guild.id, channel_id, conn, manager))
}


pub async fn leave_server(ctx: &Context<'_>) -> Result<()>{
    let guild = ctx.guild().ok_or(NotInGuildError)?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;
    let conn = manager.get(guild.id).ok_or(NotInVoiceChannelError)?;
    conn.lock().await.leave().await?;
    Ok(())
}
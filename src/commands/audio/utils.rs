use std::sync::Arc;

use poise::serenity_prelude::{GuildId, ChannelId, async_trait, Http};
use songbird::{Call, Songbird, EventContext, EventHandler as VoiceEventHandler, TrackEvent, Event};
use tokio::sync::Mutex;

use crate::commands::{Context, Result};
use crate::error::{NotInGuildError, NotInVoiceChannelError, NoVoiceChannelIdError, NoSongbirdError};

struct TrackEndHandler {
    conn: Arc<Mutex<Call>>
}

#[async_trait]
impl VoiceEventHandler for TrackEndHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            // Track(s) has ended
            if self.conn.lock().await.queue().is_empty(){
                // Nothing else in the queue
                let _ = self.conn.lock().await.leave().await;
            }
        }

        None
    }
}

// struct DisconnectHandler{
//     conn: Arc<Mutex<Call>>
// }
// #[async_trait]
// impl VoiceEventHandler for DisconnectHandler{
//     async fn act(&self, ctx: &EventContext<'_>) -> Option<Event>{
//         if let EventContext::ClientDisconnect(dc) = ctx{
//             // Another user has disconnected
//         };
//         None
//     }
// }


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
    conn.lock().await.add_global_event(
        Event::Track(TrackEvent::End),
        TrackEndHandler{conn: conn.clone()}
    );

    Ok((guild_id, channel_id, conn, manager))
}


pub async fn leave_server(ctx: &Context<'_>) -> Result<()>{
    let guild_id = ctx.guild().ok_or(NotInGuildError)?.id;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(NoSongbirdError)?;
    let conn = manager.get(guild_id).ok_or(NotInVoiceChannelError)?;
    conn.lock().await.queue().stop();
    conn.lock().await.leave().await?;
    Ok(())
}

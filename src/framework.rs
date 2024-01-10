use poise::serenity_prelude::{GatewayIntents, Activity};
use poise::{FrameworkBuilder, FrameworkError};
use songbird::SerenityInit;


use crate::utils::{get_prefix, get_status, get_token};
use crate::commands::{self, Error as CommandError};

pub struct Data {}

pub fn build() -> FrameworkBuilder<Data, CommandError>{
    let prefix = get_prefix();
    
    poise::Framework::builder()
        .options(poise::FrameworkOptions { 
            commands: commands::get(), 
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        FrameworkError::Command { error, ctx } => {
                            let _ = ctx.send(|r| {
                                r.content(error.to_string())
                            }).await;
                        }
                        error => {
                            let _ = poise::builtins::on_error(error).await;
                        }
                    }
                })
            }, 
            prefix_options: poise::PrefixFrameworkOptions { 
                prefix: Some(prefix.into()),
                mention_as_prefix: true, 
                ..Default::default()
            },
            ..Default::default()
        })
        .intents(
            GatewayIntents::non_privileged() 
            | GatewayIntents::MESSAGE_CONTENT 
            | GatewayIntents::GUILD_VOICE_STATES 
            | GatewayIntents::GUILD_MEMBERS
        )
        .token(get_token())
        .client_settings(|c| c.register_songbird())
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                if let Some(shard) = ready.shard{
                    println!("{} is connected on shard {}/{}!", ready.user.name, shard[0]+1, shard[1]);
                }
                if ctx.shard_id==0{
                    // Prevent from registering commands many times to avoid hitting the discord rate-limit for higher numbers of shards
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }
                ctx.set_activity(Activity::playing(get_status())).await;
                Ok(Data{})
            })
        })
}
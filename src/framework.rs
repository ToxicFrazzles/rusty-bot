use std::env;

use poise::{FrameworkError, Framework};
use poise::serenity_prelude::*;
use reqwest::Client as ReqwestClient;

use database::{DatabaseConnection, LogEntry};

use crate::utils::{get_prefix, get_db_url, get_status};
use crate::commands::{self, Error as CommandError};
use crate::checks::global_check;


pub struct Data {
    pub reqwest: ReqwestClient,
    pub db: mongodb::Database
}


pub async fn build() -> Framework<Data, CommandError>{
    let prefix = get_prefix();
    let mongo_conn_string = env::var("MONGODB_CONNECTION").expect("No MONGODB_CONNECTION variable set");
    let db_conn: DatabaseConnection = database::connect(mongo_conn_string).await.expect("Failed to connect to mongodb database");
    let db = db_conn.database(&env::var("MONGODB_DATABASE").expect("No MONGODB_DATABASE specified"));
    
    poise::Framework::builder()
        .options(poise::FrameworkOptions { 
            commands: commands::get(), 
            on_error: |error| {
                Box::pin(async move {
                    let db = &error.ctx().unwrap().data().db;
                    match error {
                        FrameworkError::Command { error, ctx , .. } => {
                            let _ = ctx.say(error.to_string()).await;
                            println!("Add Log");
                            LogEntry::add_log(db, 3, error.to_string()).await.unwrap();
                        },
                        error => {
                            let msg = (&error).to_string();
                            let _ = poise::builtins::on_error(error).await;
                            println!("Add Log");
                            LogEntry::add_log(db, 4, msg).await.unwrap();
                        }
                    }
                })
            }, 
            prefix_options: poise::PrefixFrameworkOptions { 
                prefix: Some(prefix.into()),
                mention_as_prefix: true, 
                ..Default::default()
            },
            skip_checks_for_owners: true,
            command_check: Some(|ctx| Box::pin(global_check(ctx))),
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                if let Some(shard) = ready.shard{
                    println!("{} is connected on shard {}/{}!", ready.user.name, shard.id.0+1, shard.total);
                }
                if ctx.shard_id.0 == 0{
                    // Prevent from registering commands many times to avoid hitting the discord rate-limit for higher numbers of shards
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }
                ctx.set_presence(
                    Some(ActivityData::custom(get_status())), 
                    OnlineStatus::Online);
                Ok(Data{
                    reqwest: ReqwestClient::new(),
                    db: db
                })
            })
        }).build()
}
use poise::{FrameworkError, Framework};
use poise::serenity_prelude::*;
use reqwest::Client as ReqwestClient;
use sea_orm::{DatabaseConnection, Database};

use migration::{Migrator, MigratorTrait};

use crate::utils::{get_prefix, get_db_url, get_status};
use crate::commands::{self, Error as CommandError};
use crate::checks::global_check;


pub struct Data {
    pub reqwest: ReqwestClient,
    pub db: DatabaseConnection
}


pub async fn build() -> Framework<Data, CommandError>{
    let prefix = get_prefix();
    let db_conn: DatabaseConnection = Database::connect(get_db_url()).await.expect("Failed to create database connection");

    let pending_migrations = Migrator::get_pending_migrations(&db_conn).await.expect("Failed to get list of pending migrations").len();
    println!("{pending_migrations} migrations pending");
    if pending_migrations > 0{
        println!("Applying migrations...");
        Migrator::up(&db_conn, None).await.expect("Failed to migrate database");
        println!("Migrations applied. Database up-to-date!");
    }
    
    poise::Framework::builder()
        .options(poise::FrameworkOptions { 
            commands: commands::get(), 
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        FrameworkError::Command { error, ctx , .. } => {
                            let _ = ctx.say(error.to_string()).await;
                        },
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
                    db: db_conn
                })
            })
        }).build()
}
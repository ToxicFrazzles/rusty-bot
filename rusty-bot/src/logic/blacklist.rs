use database::User;
use tracing::{event, Level};

use super::utils::at_to_snowflake;
use crate::error::{AlreadyBlacklistedError, Error, NotBlacklistedError, Result};


pub async fn global_set<S>(
    db: &mongodb::Database,
    who: S,
    blacklist: bool
) -> Result<()> where S: Into<String>{
    let snowflake = at_to_snowflake(who.into()).expect("Malformed user identifier");
    event!(Level::INFO, "Adding user to global blacklist");
    if let Some(user) = User::find_by_snowflake(db, &snowflake).await{
        // A user exists in the database
        event!(Level::INFO, "User exists in database. Updating the blacklist flag for them");
        let result = user.set_global_blacklist(db, blacklist).await.expect("Failed to update user record global blacklist");
        if !result && blacklist{
            return Err(Error::AlreadyBlacklisted(AlreadyBlacklistedError));
        }else if !result && !blacklist{
            return Err(Error::NotBlacklisted(NotBlacklistedError));
        }
        return Ok(());
    }
    // A user does not exist in the database
    event!(Level::INFO, "User does not exist in database. Creating them and setting blacklist flag appropriately.");
    let user = User{
        _id: snowflake.into(),
        global_blacklist: blacklist,
        ..Default::default()
    };
    if !blacklist{
        return Err(Error::NotBlacklisted(NotBlacklistedError));
    }
    user.save(db).await?;
    Ok(())
}

pub async fn global_list(db: &mongodb::Database) -> Result<Vec<String>>{
    let blacklisted = User::get_global_blacklisted_users(db).await;
    Ok(blacklisted.iter().map(|user|{
        user._id.to_string()
    }).collect())
}

pub async fn is_blacklisted<S>(
    db: &mongodb::Database,
    who: S,
    guild_id: S
) -> Result<bool> where S: Into<String>{
    let snowflake = at_to_snowflake(who.into()).expect("Malformed user identifier");
    let guild_id = at_to_snowflake(guild_id.into()).expect("Malformed guild identifier");
    
    if let Some(user) = User::find_by_snowflake(db, &snowflake).await{
        event!(Level::DEBUG, "User exists. Checking global and guild blacklist status");
        return Ok(user.global_blacklist || user.blacklisted_guilds.contains(&guild_id));
    }
    Ok(false)
}

pub async fn guild_set<S>(
    db: &mongodb::Database,
    who: S,
    guild_id: S,
    blacklist: bool
) -> Result<()> where S: Into<String>{
    let snowflake = at_to_snowflake(who.into()).expect("Malformed user identifier");
    let guild_id = at_to_snowflake(guild_id).expect("Malformed guild identifier");
    event!(Level::INFO, "Adding user to global blacklist");
    if let Some(user) = User::find_by_snowflake(db, &snowflake).await{
        // A user exists in the database
        event!(Level::INFO, "User exists in database. Updating the blacklist flag for them");
        let result = user.set_guild_blacklist(db, &guild_id, blacklist,).await.expect("Failed to update user record global blacklist");
        if !result && blacklist{
            return Err(Error::AlreadyBlacklisted(AlreadyBlacklistedError));
        }else if !result && !blacklist{
            return Err(Error::NotBlacklisted(NotBlacklistedError));
        }
        return Ok(());
    }
    // A user does not exist in the database
    event!(Level::INFO, "User does not exist in database. Creating them and setting blacklist flag appropriately.");
    let user = User{
        _id: snowflake.into(),
        blacklisted_guilds: vec![guild_id],
        ..Default::default()
    };
    if !blacklist{
        return Err(Error::NotBlacklisted(NotBlacklistedError));
    }
    user.save(db).await?;
    Ok(())
}


pub async fn guild_list<S>(db_conn: &mongodb::Database, guild_id: S) -> Result<Vec<String>>
where S: Into<String>{
    let guild_id = at_to_snowflake(guild_id).expect("Malformed guild identifier");
    let blacklisted = User::get_blacklisted_users_in_guild(db_conn, &guild_id).await;
    Ok(blacklisted.iter().map(|user|{
        user._id.to_string()
    }).collect())
}
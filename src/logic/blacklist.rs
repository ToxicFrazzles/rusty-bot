use sea_orm::{DatabaseConnection, FromQueryResult, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, ModelTrait, QuerySelect};
use entities::user::{Entity as UserEntity, Column as UserCol, ActiveModel as UserActive};
use entities::member::{Entity as MemberEntity, Column as MemberCol, ActiveModel as MemberActive};
use entities::guild::{Entity as GuildEntity, Column as GuildCol, ActiveModel as GuildActive};

use super::utils::at_to_snowflake;
use crate::error::{AlreadyBlacklistedError, Error, NotBlacklistedError, Result};

pub async fn global_add<S>(
    db_conn: &DatabaseConnection,
    who: S
) -> Result<()> where S: Into<String>{
    let snowflake = at_to_snowflake(who.into()).expect("Malformed user identifier");
    if let Some(user) = UserEntity::find()
        .filter(UserCol::Snowflake.eq(&snowflake))
        .one(db_conn)
        .await? 
    {
        if user.blacklisted{
            return Err(Error::AlreadyBlacklisted(AlreadyBlacklistedError));
        }else{
            let mut user:  UserActive = user.into();
            user.blacklisted = Set(true);
            user.update(db_conn).await?;
        }
    }else{
        let user = UserActive{
            snowflake: Set(snowflake),
            blacklisted: Set(true),
            ..Default::default()
        };
        let _ = UserEntity::insert(user).exec(db_conn).await?;
    }
    Ok(())
}


pub async fn global_remove<S>(
    db_conn: &DatabaseConnection,
    who: S
) -> Result<()> where S: Into<String>{
    let snowflake = at_to_snowflake(who.into()).expect("Malformed user identifier");
    if let Some(user) = UserEntity::find()
        .filter(UserCol::Snowflake.eq(&snowflake))
        .one(db_conn)
        .await? 
    {
        if !user.blacklisted{
            return Err(Error::NotBlacklisted(NotBlacklistedError));
        }else{
            let mut user:  UserActive = user.into();
            user.blacklisted = Set(false);
            user.update(db_conn).await?;
        }
    }else{
        return Err(Error::NotBlacklisted(NotBlacklistedError));
    }
    Ok(())
}


pub async fn is_blacklisted<S>(
    db_conn: &DatabaseConnection,
    who: S,
    guild_id: S
) -> Result<bool> where S: Into<String>{
    let snowflake = at_to_snowflake(who.into()).expect("Malformed user identifier");
    let user = UserEntity::find()
    .filter(UserCol::Snowflake.eq(&snowflake))
    .one(db_conn)
    .await?;
    if user == None{
        return Ok(false);
    }
    let user = user.unwrap();
    if user.blacklisted{
        return Ok(true);
    }

    if let Some(member) = user.find_related(MemberEntity).left_join(GuildEntity).filter(GuildCol::Snowflake.eq(&guild_id.into())).one(db_conn).await? {
        if member.blacklisted{
            return Ok(true);
        }
    }
    return Ok(false);
}


pub async fn guild_add<S>(
    db_conn: &DatabaseConnection,
    who: S,
    guild_id: S
) -> Result<()> where S: Into<String>{
    todo!("The whole guild_add method");
    let snowflake = at_to_snowflake(who).expect("Malformed user identifier");
    let user = UserEntity::find()
        .filter(UserCol::Snowflake.eq(&snowflake))
        .one(db_conn)
        .await?;
    if user == None{
        let user = UserActive{
            snowflake: Set(snowflake),
            ..Default::default()
        };
        UserEntity::insert(user).exec(db_conn).await?;
    }

    
    if let Some(user) = UserEntity::find()
        .filter(UserCol::Snowflake.eq(&snowflake))
        .one(db_conn)
        .await? 
    {
        if user.blacklisted{
            return Err(Error::AlreadyBlacklisted(AlreadyBlacklistedError));
        }else{
            let mut user:  UserActive = user.into();
            user.blacklisted = Set(true);
            user.update(db_conn).await?;
        }
    }else{
        let user = UserActive{
            snowflake: Set(snowflake),
            blacklisted: Set(true),
            ..Default::default()
        };
        let _ = UserEntity::insert(user).exec(db_conn).await?;
    }
    Ok(())
}


pub async fn guild_remove<S>(
    db_conn: &DatabaseConnection,
    who: S,
    guild_id: S
) -> Result<()> where S: Into<String>{
    todo!("The whole guild_remove method");
    let snowflake = at_to_snowflake(who).expect("Malformed user identifier");
    if let Some(user) = UserEntity::find()
        .filter(UserCol::Snowflake.eq(&snowflake))
        .one(db_conn)
        .await? 
    {
        if !user.blacklisted{
            return Err(Error::NotBlacklisted(NotBlacklistedError));
        }else{
            let mut user:  UserActive = user.into();
            user.blacklisted = Set(false);
            user.update(db_conn).await?;
        }
    }else{
        return Err(Error::NotBlacklisted(NotBlacklistedError));
    }
    Ok(())
}

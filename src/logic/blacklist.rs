use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, ModelTrait, IntoActiveModel};
use entities::user::{Entity as UserEntity, Column as UserCol, ActiveModel as UserActive};
use entities::member::{Entity as MemberEntity, Column as MemberCol, ActiveModel as MemberActive};
use entities::guild::{Entity as GuildEntity, Column as GuildCol, ActiveModel as GuildActive};

use super::utils::at_to_snowflake;
use crate::error::{AlreadyBlacklistedError, Error, NotBlacklistedError, Result};


pub async fn global_set<S>(
    db_conn: &DatabaseConnection,
    who: S,
    blacklist: bool
) -> Result<()> where S: Into<String>{
    let snowflake = at_to_snowflake(who.into()).expect("Malformed user identifier");
    if let Some(user) = UserEntity::find()
        .filter(UserCol::Id.eq(snowflake))
        .one(db_conn)
        .await? 
    {
        if user.blacklisted && blacklist{
            return Err(Error::AlreadyBlacklisted(AlreadyBlacklistedError));
        }else if !user.blacklisted && !blacklist{
            return Err(Error::NotBlacklisted(NotBlacklistedError));
        }
        let mut user:  UserActive = user.into();
        user.blacklisted = Set(blacklist);
        user.update(db_conn).await?;
        return Ok(());
    }
    if !blacklist{
        return Err(Error::NotBlacklisted(NotBlacklistedError));
    }
    let user = UserActive{
        id: Set(snowflake),
        blacklisted: Set(blacklist),
        ..Default::default()
    };
    let _ = UserEntity::insert(user).exec(db_conn).await?;

    Ok(())
}

pub async fn global_list(db_conn: &DatabaseConnection) -> Result<Vec<String>>{
    let users = UserEntity::find()
        .filter(UserCol::Blacklisted.eq(true))
        .all(db_conn)
        .await?;

    Ok(users.iter().map(|user|{
        (user.id as u64).to_string()
    })
    .collect())
}

pub async fn is_blacklisted<S>(
    db_conn: &DatabaseConnection,
    who: S,
    guild_id: S
) -> Result<bool> where S: Into<String>{
    let snowflake = at_to_snowflake(who.into()).expect("Malformed user identifier");
    let guild_id = at_to_snowflake(guild_id.into()).expect("Malformed guild identifier");
    let user = UserEntity::find()
    .filter(UserCol::Id.eq(snowflake))
    .one(db_conn)
    .await?;
    if user == None{
        return Ok(false);
    }
    let user = user.unwrap();
    if user.blacklisted{
        return Ok(true);
    }

    if let Some(member) = user.find_related(MemberEntity).left_join(GuildEntity).filter(GuildCol::Id.eq(guild_id)).one(db_conn).await? {
        if member.blacklisted{
            return Ok(true);
        }
    }
    return Ok(false);
}

pub async fn guild_set<S>(
    db_conn: &DatabaseConnection,
    who: S,
    guild_id: S,
    blacklist: bool
) -> Result<()> where S: Into<String>{
    let snowflake = at_to_snowflake(who).expect("Malformed user identifier");
    let guild_id = at_to_snowflake(guild_id).expect("Malformed guild identifier");

    let user = UserEntity::find()
        .filter(UserCol::Id.eq(snowflake)).one(db_conn).await?;
    if user == None && blacklist {
        // Create user and member
        UserEntity::insert(UserActive{
            id: Set(snowflake),
            ..Default::default()
        }).exec(db_conn).await?;
        let member = MemberActive{
            user: Set(snowflake),
            guild: Set(guild_id),
            blacklisted: Set(blacklist),
            ..Default::default()
        };
        return match MemberEntity::insert(member.clone()).exec(db_conn).await{
            Ok(_) => Ok(()),
            Err(_) => {
                GuildEntity::insert(GuildActive{
                    id: Set(guild_id),
                    ..Default::default()
                }).exec(db_conn).await?;
                MemberEntity::insert(member).exec(db_conn).await?;
                Ok(())
            }
        };

    }else if user == None {
        return Err(Error::NotBlacklisted(NotBlacklistedError));
    }

    let user = user.unwrap();
    let member = user.find_related(MemberEntity).filter(MemberCol::Guild.eq(guild_id)).one(db_conn).await?;
    if member == None && blacklist{
        let member = MemberActive{
            user: Set(snowflake),
            guild: Set(guild_id),
            blacklisted: Set(blacklist),
            ..Default::default()
        };
        return match MemberEntity::insert(member.clone()).exec(db_conn).await{
            Ok(_) => Ok(()),
            Err(_) => {
                GuildEntity::insert(GuildActive{
                    id: Set(guild_id),
                    ..Default::default()
                }).exec(db_conn).await?;
                MemberEntity::insert(member).exec(db_conn).await?;

                Ok(())
            }
        };
    }else if member == None{
        return Err(Error::NotBlacklisted(NotBlacklistedError));
    }

    let mut member = member.unwrap().into_active_model();
    let blacklisted = member.clone().blacklisted.unwrap();
    
    if blacklisted && blacklist{
        return Err(Error::AlreadyBlacklisted(AlreadyBlacklistedError));
    }else if !blacklisted && !blacklist {
        return Err(Error::NotBlacklisted(NotBlacklistedError));
    }
    member.blacklisted = Set(blacklist);
    member.save(db_conn).await?;

    Ok(())
}


pub async fn guild_list<S>(db_conn: &DatabaseConnection, guild_id: S) -> Result<Vec<String>>
where S: Into<String>{
    let guild_id = at_to_snowflake(guild_id).unwrap();
    let members = MemberEntity::find()
        .filter(MemberCol::Guild.eq(guild_id))
        .filter(MemberCol::Blacklisted.eq(true))
        .all(db_conn)
        .await?;

    Ok(members.iter().map(|member|{
        (member.user as u64).to_string()
    })
    .collect())
}
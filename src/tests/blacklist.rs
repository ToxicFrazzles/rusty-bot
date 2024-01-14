use sea_orm::Database;
use sea_orm::{
    entity::prelude::*, entity::*, //tests_cfg::*,
    DatabaseBackend, MockDatabase, Transaction,
};
use crate::error::{Error, Result};
use crate::logic::blacklist::{global_set, is_blacklisted, guild_set};
use entities::user::{Model as UserModel};
use entities::member::{Entity as MemberEntity, Column as MemberCol, ActiveModel as MemberActive};
use entities::guild::{Entity as GuildEntity, Column as GuildCol, ActiveModel as GuildActive};
use migration::{Migrator, MigratorTrait};


async fn test_db_setup() -> Result<DatabaseConnection>{
    let db = Database::connect("sqlite::memory:").await?;
    Migrator::up(&db, None).await?;
    return Ok(db)
}


#[tokio::test]
async fn test_global_blacklist() -> Result<()>{
    let db = test_db_setup().await.unwrap();
    let user_id = &"<@169536101357191168>".to_string();
    let guild_id = &"316250642194628608".to_string();
    // Test 1: Check non-existent user is not blacklisted
    println!("Test 1");
    assert!(!is_blacklisted(&db, user_id, guild_id).await.unwrap());

    // Test 2: Blacklist non-existent user
    println!("Test 2");
    assert!(
        match global_set(&db, user_id, true).await{
            Ok(_) => true,
            Err(why) => {
                println!("{:?}", why);
                false
            }
        }
    );

    // Test 3: Check freshly blacklisted user is still blacklisted
    println!("Test 3");
    assert!(is_blacklisted(&db, user_id, guild_id).await.unwrap());

    // Test 4: Blacklist blacklisted user
    println!("Test 4");
    assert!(
        match global_set(&db, user_id, true).await{
            Ok(_) => false,
            Err(_) => true
        }
    );

    // Test 5: remove user from blacklist
    println!("Test 5");
    assert!(
        match global_set(&db, user_id, false).await{
            Ok(_) => true,
            Err(_) => false
        }
    );

    // Test 6: Test freshly un-blacklisted user is no longer blacklisted
    println!("Test 6");
    assert!(!is_blacklisted(&db, user_id, guild_id).await.unwrap());

    // Test 7: remove user not on blacklist from blacklist
    println!("Test 7");
    assert!(
        match global_set(&db, user_id, false).await{
            Ok(_) => false,
            Err(_) => true
        }
    );

    // Test 8: Blacklist existing user
    println!("Test 8");
    assert!(
        match global_set(&db, user_id, true).await{
            Ok(_) => true,
            Err(_) => false
        }
    );

    // Test 9: Check freshly blacklisted user is still blacklisted
    println!("Test 9");
    assert!(is_blacklisted(&db, user_id, guild_id).await.unwrap());

    Ok(())
}



#[tokio::test]
async fn test_guild_blacklist() -> Result<()>{
    let db = test_db_setup().await.unwrap();
    let user_id = &"<@169536101357191168>".to_string();
    let guild_id = &"316250642194628608".to_string();

    // Test 1: Nonexistent user shouldn't be blacklisted
    assert!(!is_blacklisted(&db, user_id, guild_id).await.unwrap());

    // Test 2: Unblacklist nonexistent user
    assert!(
        match guild_set(&db, user_id, guild_id, false).await{
            Ok(_) => false,
            Err(_) => true
        }
    );

    // Test 3: Blacklist nonexistent user with a nonexistent guild
    assert!(
        match guild_set(&db, user_id, guild_id, true).await{
            Ok(_) => true,
            Err(_) => false
        }
    );

    // Test 4: User should be blacklisted
    assert!(is_blacklisted(&db, user_id, guild_id).await.unwrap());

    // Test 5: Un-blacklist user
    assert!(
        match guild_set(&db, user_id, guild_id, false).await{
            Ok(_) => true,
            Err(_) => false
        }
    );

    // Test 6: User should not be blacklisted
    assert!(!is_blacklisted(&db, user_id, guild_id).await.unwrap());

    Ok(())
}
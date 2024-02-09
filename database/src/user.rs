use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};
use crate::types::Snowflake;


const SCHEMA_VER: i64 = 1;
pub const USER_COLLECTION: &str = "users";

#[derive(Serialize, Deserialize)]
pub struct User{
    pub _id: Snowflake,
    #[serde(default)]
    pub schema_ver: i64,
    pub global_blacklist: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub blacklisted_guilds: Vec<Snowflake>,
}

impl Default for User{
    fn default() -> Self {
        User { 
            _id: "".into(),
            schema_ver: SCHEMA_VER,
            global_blacklist: false,
            blacklisted_guilds: vec![]
        }
    }
}


impl User {
    pub async fn find_by_snowflake(db: &mongodb::Database, snowflake: &Snowflake) -> Option<User>{
        let coll = db.collection::<User>(USER_COLLECTION);
        let oid: ObjectId = snowflake.clone().into();
        coll.find_one(doc!{"_id": oid}, None).await.expect("Error finding record")
    }

    pub async fn find_and_delete_by_snowflake(db: &mongodb::Database, snowflake: &Snowflake) -> Option<User>{
        let coll = db.collection::<User>(USER_COLLECTION);
        let oid: ObjectId = snowflake.clone().into();
        coll.find_one_and_delete(doc!{"_id": oid}, None).await.expect("Error finding record")
    }

    pub async fn set_global_blacklist(&self, db: &mongodb::Database, blacklist: bool)->mongodb::error::Result<bool>{
        let coll = db.collection::<User>(USER_COLLECTION);
        let oid: ObjectId = self._id.clone().into();
        let res = coll.update_one(doc!{"_id": oid}, doc!{
            "$set": doc!{
                "global_blacklist": blacklist
            }
        }, None).await?;
        Ok(res.modified_count > 0)
    }

    pub async fn set_guild_blacklist(&self, db: &mongodb::Database, guild: &Snowflake, blacklist: bool)->mongodb::error::Result<bool>{
        let coll = db.collection::<User>(USER_COLLECTION);
        let oid: ObjectId = self._id.clone().into();
        let guild_oid: ObjectId = guild.clone().into();
        if blacklist{
            let res = coll.update_one(doc!{"_id": oid}, doc!{
                "$addToSet": doc!{
                    "blacklisted_guilds": guild_oid
                }
            }, None).await?;
            return Ok(res.modified_count > 0);
        }
        let res = coll.update_one(doc!{"_id": oid}, doc!{
            "$pull": doc!{
                "blacklisted_guilds": guild_oid
            }
        }, None).await?;
        Ok(res.modified_count > 0)
    }

    pub async fn save(&self, db: &mongodb::Database)->mongodb::error::Result<()>{
        let coll = db.collection::<User>(USER_COLLECTION);
        let oid: ObjectId = self._id.clone().into();
        let res = coll.replace_one(doc!{"_id": oid}, self, None).await?;
        if res.modified_count == 0{
            coll.insert_one(self, None).await?;
        }
        Ok(())
    }

    pub async fn get_blacklisted_users_in_guild(db: &mongodb::Database, guild: &Snowflake)->Vec<User>{
        let coll = db.collection::<User>(USER_COLLECTION);
        let oid: ObjectId = guild.clone().into();
        let cursor = coll.find(doc! {"blacklisted_guilds": oid}, None).await.unwrap();
        cursor.try_collect().await.unwrap()
    }

    pub async fn get_global_blacklisted_users(db: &mongodb::Database)->Vec<User>{
        let coll = db.collection::<User>(USER_COLLECTION);
        let cursor = coll.find(doc! {"global_blacklist": true}, None).await.unwrap();
        cursor.try_collect().await.unwrap()
    }
}


#[tokio::test]
async fn test_user_insert_and_select(){
    use crate::connect;
    let client = connect("mongodb://localhost:27017/".to_string()).await.expect("Failed to connect to database");

    let db = &client.database("rusty-bot-test");
    let coll = db.collection::<User>("users");

    let test_snowflake: Snowflake = "169536101357191168".into();

    coll.insert_one(User{
        _id: test_snowflake.clone(),
        ..Default::default()
    }, None).await.expect("Failed to insert record into database");

    let selected = User::find_and_delete_by_snowflake(db, &test_snowflake).await.expect("Could not find record");

    assert_eq!(selected.schema_ver, SCHEMA_VER);
    assert!(selected.blacklisted_guilds.is_empty());
}
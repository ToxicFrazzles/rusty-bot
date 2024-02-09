use mongodb::bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};

use crate::{
    types::Snowflake,
    Result,
};


const SCHEMA_VER: i64 = 1;
const GUILD_COLLECTION: &str = "guilds";


#[derive(Serialize, Deserialize)]
pub struct Guild{
    pub _id: Snowflake,
    pub schema_ver: i64,
    pub vc_timeout: i64,
}

impl Default for Guild{
    fn default() -> Self {
        Guild { 
            _id: "".into(),
            schema_ver: SCHEMA_VER,
            vc_timeout: 10,
        }
    }
}


impl Guild {
    pub async fn find_by_snowflake(db: &mongodb::Database, snowflake: &str) -> Option<Guild>{
        let coll = db.collection::<Guild>(GUILD_COLLECTION);
        let oid: ObjectId = Snowflake::from(snowflake).into();
        coll.find_one(doc!{"_id": oid}, None).await.expect("Error finding record")
    }
    pub async fn find_by_snowflake_or_default(db: &mongodb::Database, snowflake: &str) -> Guild{
        let coll = db.collection::<Guild>(GUILD_COLLECTION);
        let oid: ObjectId = Snowflake::from(snowflake).into();
        let res = coll.find_one(doc!{"_id": oid}, None).await.expect("Error finding record");
        if res.is_none(){
            return Guild{
                _id: snowflake.into(),
                ..Default::default()
            };
        }
        res.unwrap()
    }

    pub async fn find_and_delete_by_snowflake(db: &mongodb::Database, snowflake: &str) -> Option<Guild>{
        let coll = db.collection::<Guild>(GUILD_COLLECTION);
        let oid: ObjectId = Snowflake::from(snowflake).into();
        coll.find_one_and_delete(doc!{"_id": oid}, None).await.expect("Error finding record")
    }

    pub async fn set_vc_timeout(db: &mongodb::Database, guild_id: &str, vc_timeout: i64) -> Result<()>{
        let coll = db.collection::<Guild>(GUILD_COLLECTION);
        let snowflake: Snowflake = guild_id.into();
        let oid: ObjectId = Snowflake::from(guild_id).into();
        let res = coll.find_one_and_update(doc!{
            "_id": oid
        }, 
        doc!{
            "$set": {
                "vc_timeout": vc_timeout,
            }
        }, None).await?;
        if res.is_none(){
            let guild = Guild{
                _id: snowflake,
                vc_timeout: vc_timeout,
                ..Default::default()
            };
            coll.insert_one(guild,None).await?;
        }
        Ok(())
    }
}


#[tokio::test]
async fn test_guild_insert_and_select(){
    use crate::connect;
    let client = connect("mongodb://localhost:27017/".to_string()).await.expect("Failed to connect to database");

    let db = &client.database("rusty-bot-test");
    let coll = db.collection::<Guild>("guilds");

    coll.insert_one(Guild{
        _id: "169841031456489472".into(),
        ..Default::default()
    }, None).await.expect("Failed to insert record into database");

    let selected = Guild::find_and_delete_by_snowflake(db, "169841031456489472").await.expect("Could not find record");

    assert_eq!(selected.schema_ver, SCHEMA_VER);
}
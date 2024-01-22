use mongodb::bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};

use crate::types::Snowflake;
// use crate::helpers::snowflake_as_oid::{self, snowflake_to_oid};


const SCHEMA_VER: i64 = 1;


#[derive(Serialize, Deserialize)]
pub struct Guild{
    _id: Snowflake,
    schema_ver: i64,
}

impl Default for Guild{
    fn default() -> Self {
        Guild { 
            _id: "".into(),
            schema_ver: SCHEMA_VER
        }
    }
}


impl Guild {
    pub async fn find_by_snowflake(db: mongodb::Database, snowflake: &str) -> Option<Guild>{
        let coll = db.collection::<Guild>("guilds");
        let oid: ObjectId = Snowflake::from(snowflake).into();
        coll.find_one(doc!{"_id": oid}, None).await.expect("Error finding record")
    }

    pub async fn find_and_delete_by_snowflake(db: mongodb::Database, snowflake: &str) -> Option<Guild>{
        let coll = db.collection::<Guild>("guilds");
        let oid: ObjectId = Snowflake::from(snowflake).into();
        coll.find_one_and_delete(doc!{"_id": oid}, None).await.expect("Error finding record")
    }
}


#[tokio::test]
async fn test_guild_insert_and_select(){
    use crate::connect;
    let client = connect("mongodb://localhost:27017/".to_string()).await.expect("Failed to connect to database");

    let db = client.database("rusty-bot-test");
    let coll = db.collection::<Guild>("guilds");

    coll.insert_one(Guild{
        _id: "169841031456489472".into(),
        ..Default::default()
    }, None).await.expect("Failed to insert record into database");

    let selected = Guild::find_and_delete_by_snowflake(db, "169841031456489472").await.expect("Could not find record");

    assert_eq!(selected.schema_ver, SCHEMA_VER);
}
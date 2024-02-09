use chrono::Utc;
use mongodb::{bson::{doc, serde_helpers::i64_as_bson_datetime}, Database};
use serde::{Serialize, Deserialize};


const SCHEMA_VER: i64 = 1;
pub const LOG_ENTRY_COLLECTION: &str = "log_entries";

#[derive(Serialize, Deserialize)]
pub struct LogEntry{
    #[serde(default)]
    pub schema_ver: i64,
    #[serde(with="i64_as_bson_datetime")]
    pub time: i64,
    pub level: i8,
    pub text: String,
}

impl Default for LogEntry{
    fn default() -> Self {
        LogEntry { 
            schema_ver: SCHEMA_VER,
            time: Utc::now().timestamp_millis(),
            level: 1,
            text: "".to_string(),
        }
    }
}

impl LogEntry{
    pub async fn add_log(db: &Database, level: i8, message: String)->mongodb::error::Result<()>{
        let coll = db.collection::<LogEntry>(LOG_ENTRY_COLLECTION);
        let new_entry = LogEntry{
            level: level,
            text: message,
            ..Default::default()
        };
        coll.insert_one(new_entry, None).await?;
        Ok(())
    }
}
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::types::Snowflake;


const SCHEMA_VER: i64 = 1;
pub const OAUTH_COLLECTION: &str = "oauth";

#[derive(Serialize)]
pub struct Oauth{
    #[serde(default)]
    schema_ver: i64,
    pub user: Option<Snowflake>,
    pub session_token_hash: String,

    pub state: String,

    pub access_token: Option<String>,
    pub access_expires: Option<i64>,
    pub refresh_token: Option<String>,
    pub refresh_expires: Option<i64>,

    pub created_at: i64,
    pub modified: i64,
}


impl Oauth{
    pub fn gen_state() -> String{
        let rng = thread_rng();
        let v: Vec<u8> = rng.sample_iter(&Alphanumeric).take(40).collect();
        String::from_utf8(v).unwrap()
    }

    pub fn new() -> Oauth{

        todo!()
    }


}
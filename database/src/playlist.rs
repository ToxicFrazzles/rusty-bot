use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Serialize, Deserialize};
use crate::types::Snowflake;

const SCHEMA_VER: i64 = 1;
pub const PLAYLIST_COLLECTION: &str = "playlists";



#[derive(Serialize, Deserialize)]
pub struct Song{
    url: String,
}


#[derive(Serialize, Deserialize)]
pub enum PlaylistVisibility{
    User,
    Guild,
    Global,
}


#[derive(Serialize, Deserialize)]
pub struct Playlist{
    #[serde(default)]
    pub schema_ver: i64,
    pub visibility: PlaylistVisibility,
    pub snowflake: Option<Snowflake>,
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub songs: Vec<Song>,
}

impl Default for Playlist{
    fn default() -> Self {
        Playlist { 
            name: "".to_string(),
            visibility: PlaylistVisibility::Global,
            snowflake: None,
            schema_ver: SCHEMA_VER,
            songs: vec![],
        }
    }
}


impl Playlist{
    pub async fn get_playlists(db: &mongodb::Database, user_id: Snowflake, guild_id: Snowflake) -> mongodb::error::Result<Vec<Playlist>>{
        todo!("Implement method to get all available playlists")
    }

    pub async fn add_playlist(db: &mongodb::Database, visibility: PlaylistVisibility, snowflake: Option<Snowflake>, name: String, songs: Vec<String>) -> mongodb::error::Result<()>{
        todo!()
    }

    pub async fn add_user_playlist(db: &mongodb::Database, user_id: Snowflake, name: String, songs: Vec<String>) -> mongodb::error::Result<()>{
        todo!()
    }

    pub async fn add_guild_playlist(db: &mongodb::Database, guild_id: Snowflake, name: String, songs: Vec<String>) -> mongodb::error::Result<()>{
        todo!()
    }

    pub async fn add_global_playlist(db: &mongodb::Database, name: String, songs: Vec<String>) -> mongodb::error::Result<()>{
        todo!()
    }

    pub async fn add_song_to_playlist(db: &mongodb::Database, song_url: &str) -> mongodb::error::Result<()>{
        todo!()
    }
}
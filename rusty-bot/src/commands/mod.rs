use crate::framework::Data;

pub mod audio;
pub mod ping;
pub mod help;
pub mod blacklist;
pub mod info;
pub mod config;

pub use crate::error::Error;
pub use crate::error::Result;

pub type Context<'a> = poise::Context<'a, Data, Error>;


pub fn get() -> Vec<poise::Command<Data, Error>>{
    vec![
        ping::ping(),
        
        audio::play::play(),
        audio::skip::skip(),
        audio::leave::leave(),
        audio::join::join(),

        // config::config(),

        blacklist::blacklist(),
        help::help(),
        info::info(),
    ]
}
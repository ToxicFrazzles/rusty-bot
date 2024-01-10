use crate::framework::Data;

pub mod audio;
pub mod ping;
pub mod help;

mod error;

pub use self::error::Error;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Result<R> = core::result::Result<R, Error>;



pub fn get() -> Vec<poise::Command<Data, Error>>{
    vec![
        ping::ping(),
        audio::play::play(),
        audio::skip::skip(),
        audio::leave::leave(),
        help::help(),
    ]
}
use songbird::{
    error::JoinError as SongbirdJoinError,
    // input::error::Error as SongbirdInputError
};
use thiserror::Error;


#[derive(Debug, Error)]
#[error("Not in a guild")]
pub struct NotInGuildError;

#[derive(Debug, Error)]
#[error("Not in a voice channel")]
pub struct NotInVoiceChannelError;

#[derive(Debug, Error)]
#[error("Voice channel has no ID")]
pub struct NoVoiceChannelIdError;

#[derive(Debug, Error)]
#[error("Songbird hasn't been initialised (I'm stupid, this is a bug)")]
pub struct NoSongbirdError;

#[derive(Debug, Error)]
#[error("User already blacklisted")]
pub struct AlreadyBlacklistedError;

#[derive(Debug, Error)]
#[error("User not blacklisted")]
pub struct NotBlacklistedError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Serenity(#[from] serenity::Error),

    #[error(transparent)]
    NotInGuild(#[from] NotInGuildError),

    #[error(transparent)]
    NotInVoiceChannel(#[from] NotInVoiceChannelError),

    #[error(transparent)]
    NoVoiceChannelId(#[from] NoVoiceChannelIdError),

    #[error(transparent)]
    NoSongbird(#[from] NoSongbirdError),

    #[error(transparent)]
    SongbirdJoin(#[from] SongbirdJoinError),

    // #[error(transparent)]
    // SongbirdInput(#[from] SongbirdInputError),

    #[error(transparent)]
    Database(#[from] mongodb::error::Error),

    #[error(transparent)]
    AlreadyBlacklisted(#[from] AlreadyBlacklistedError),

    #[error(transparent)]
    NotBlacklisted(#[from] NotBlacklistedError),
}



pub type Result<R> = core::result::Result<R, Error>;
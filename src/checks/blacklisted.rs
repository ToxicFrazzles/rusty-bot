use crate::{logic::blacklist::is_blacklisted, commands::Context};

use crate::error::Error;


pub async fn blacklisted(ctx: Context<'_>) -> Result<bool, Error>{
        let mut allowed: bool = true;
        let db = &ctx.data().db;
        let who = ctx.author().id.to_string();
        let guild_id = ctx.guild_id().expect("No guild ID").to_string();

        allowed &= is_blacklisted(db, who, guild_id).await.unwrap();

        Ok(allowed)
}
use crate::{commands::Context, error::Error};

use super::blacklisted::blacklisted;



pub async fn global_check(ctx: Context<'_>) -> Result<bool, Error>{
        let mut allowed: bool = true;
        allowed &= !blacklisted(ctx).await.unwrap();
        Ok(allowed)
}
use rocket::{serde::Serialize, *};


#[derive(Serialize)]
struct DiscordLoginInfoResp{
    client_id: String,
}

#[get("/discord_auth")]
pub fn discord_login_info() -> String{
    let discord_id = std::env::var("DISCORD_ID").expect("Cannot return login URL without a discord ID set...");

    let resp = DiscordLoginInfoResp{
        client_id: discord_id
    };
    serde_json::to_string(&resp).unwrap()
}
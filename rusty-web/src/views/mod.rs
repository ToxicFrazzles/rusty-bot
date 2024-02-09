use rocket::*;

mod discord_login;




#[get("/")]
fn index() -> &'static str{
    "Hello, World!"
}



pub fn get_routes() -> Vec<Route>{
    routes![
        index, discord_login::discord_login_info
    ]
}
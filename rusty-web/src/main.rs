use rocket::*;

mod session_fairing;
mod views;



#[launch]
fn rocket() -> _ {
    // Load .env file if it exists. Falls back to loading the variables from the actual environment
    dotenv::dotenv().ok();
    rocket::build()
        .attach(session_fairing::SessionFairing{})
        .mount("/", views::get_routes())
}

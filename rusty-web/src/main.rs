use rocket::*;
use rocket_dyn_templates::{context, Template};


#[get("/")]
fn index() -> Template{
    Template::render("index", context!{})
}


#[launch]
fn rocket() -> _ {
    // Load .env file if it exists. Falls back to loading the variables from the actual environment
    dotenv::dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

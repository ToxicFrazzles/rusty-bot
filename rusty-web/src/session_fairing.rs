use rocket::fairing::{Fairing, Info, Kind};

pub struct SessionFairing{

}


#[rocket::async_trait]
impl Fairing for SessionFairing{
    fn info(&self) -> Info {
        Info{
            name: "Session Token Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r rocket::Request<'_>, _res: &mut rocket::Response<'r>){
        // TODO: Check if request has a session token cookie. 
        // If no session token cookie and not an error page: add session token cookie to response
        return
    }
}
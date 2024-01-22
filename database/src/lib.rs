use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client, bson::doc};

pub use mongodb::{Client as DatabaseConnection, Database};

// mod helpers;
mod user;
mod guild;
pub mod types;
pub use user::User;
pub use guild::Guild;


pub async fn connect(conn_string: String) -> mongodb::error::Result<DatabaseConnection>{
    let mut client_options =
        ClientOptions::parse(conn_string)
          .await.expect("Error creating mongodb client options");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).expect("Failed to create mongodb client");
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await.expect("Failed to ping the deployment");

    Ok(client)
}

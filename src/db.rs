use std::env;
use cached::proc_macro::cached;
use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client };

#[allow(dead_code)]
#[cached(time = 86400)]
async fn get_connection() -> mongodb::error::Result<Client> {
    let uri = env::var("MONGO_URI").expect("Please set your MONGO_URI environment variable");
    let mut client_options = ClientOptions::parse(uri).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();

    client_options.server_api = Some(server_api);
    Client::with_options(client_options)
}
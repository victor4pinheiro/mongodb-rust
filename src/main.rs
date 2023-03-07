mod model;
mod controller;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use mongodb::Client;
use controller::user::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .configure(routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{get, web, App, HttpServer, Responder};
use mongodb::{options::ClientOptions, Client};
use std::{sync::*, env};
mod logs_handlers;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let mut arr = Vec::new();
    for i in 0..10000 + 1 {
        let mut sum: u128 = 0;
        for j in 0..i + 1 {
            sum = sum + j;
        }
        arr.push(sum);
    }
    let total = arr.len();
    format!("Hello {total}!")

}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb+srv://admin:ergon2021@dev.cwik3.mongodb.net/ergonglobal?retryWrites=true&w=majority
    ").await.unwrap();
    client_options.app_name = Some("MS".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(greet)
            .service(web::scope("/api").configure(logs_handlers::scoped_config))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
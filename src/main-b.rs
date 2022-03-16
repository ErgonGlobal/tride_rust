use actix_web::{
    get, http, middleware, post, web, App, HttpMessage, HttpResponse, HttpServer, Responder,
};
use serde::Serialize;

mod middlewares;
use crate::middlewares::auth::AuthorizationService;
use reqwest::{Client, StatusCode, Url};
use std::collections::HashMap;
use std::{env, sync::*};
mod logs_handlers;

#[derive(Serialize)]
struct Country {
    country_code: String,
    country_name: String,
}

async fn redirect(req: AuthorizationService) -> HttpResponse {
    let user = req.user;

    /*
    let client = Client::new();
    let mut res = client.get("http://localhost:3000".to_owned() + &req.path);

    let res = res
        .header("USER", user)
        .send()
        .await
        .expect("failed to get response")
        .json::<serde_json::Value>()
        .await
        .expect("failed to get payload");

    HttpResponse::Ok().json(res)
    */

    
    return HttpResponse::Found()
        .status(http::StatusCode::TEMPORARY_REDIRECT)
        .append_header(("X-Request-ID", user))
        .append_header(("Location", "http://localhost:3000".to_owned() + &req.path))

        .finish();
        
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().default_service(web::route().to(redirect)))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}

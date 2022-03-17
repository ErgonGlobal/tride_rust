use actix_web::{
    get, http, middleware, post, web, App, HttpMessage, HttpResponse, HttpServer, Responder,
};
mod middlewares;
use crate::middlewares::auth::AuthorizationService;
use reqwest::{Client, StatusCode, Url};

async fn redirect(req: AuthorizationService) -> HttpResponse {
    let user = req.user;
    println!("{}", "http://data-service.default.svc.cluster.local".to_owned() + &req.path);

    let client = Client::new();
    let res = client.get("http://data-service.default.svc.cluster.local".to_owned() + &req.path);

    let res = res
        .header("USER", user)
        .send()
        .await
        .expect("failed to get response")
        .json::<serde_json::Value>()
        .await
        .expect("failed to get payload");

    return HttpResponse::Ok().json(res);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().default_service(web::route().to(redirect)))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}

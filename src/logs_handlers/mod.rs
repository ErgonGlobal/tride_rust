use crate::middlewares::auth::AuthorizationService;
use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::FindOptions,
    results, Client,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const MONGO_DB: &'static str = "ergonglobal";
const MONGO_COLL_LOGS: &'static str = "bountyads";

#[derive(Serialize, Deserialize)]
pub struct Res  {
     data: Vec<Document>,
     token:String,
}

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/logs")
            .route(web::get().to(get_logs))
            .route(web::post().to(add_log)),
    );
}

async fn get_logs(req: AuthorizationService ,data: web::Data<Mutex<Client>>) -> impl Responder {
    let logs_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection::<Document>(MONGO_COLL_LOGS);

    let filter = doc! {};
    let find_options = FindOptions::builder().build();
    let mut cursor = logs_collection.find(filter, find_options).await.unwrap();

    let mut results = Vec::new();
    //  let results: Vec<Result<Document, Error>> = cursor.collect().await;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    
    let res: Res = {
        Res {
            data: results,
            token: req.user,
        }
    };

    HttpResponse::Ok().json(res)
}

async fn add_log() -> impl Responder {
    format!("Not yet implemented!")
}

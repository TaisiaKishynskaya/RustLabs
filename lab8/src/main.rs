use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;
use dashmap::DashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Serialize)]
struct ResponseData {
    query: String,
    result: String,
}

type Cache = Arc<DashMap<String, String>>;

async fn compute_handler(
    query: web::Query<std::collections::HashMap<String, String>>,
    cache: web::Data<Cache>,
) -> impl Responder {
    let input = query.get("input").cloned().unwrap_or_default();

    if let Some(cached) = cache.get(&input) {
        return HttpResponse::Ok().json(ResponseData {
            query: input,
            result: cached.clone(),
        });
    }

    let input_clone = input.clone();
    let result = web::block(move || {
        thread::sleep(Duration::from_millis(100));
        format!("Processed: {}", input_clone)
    })
        .await
        .unwrap();

    cache.insert(input.clone(), result.clone());

    HttpResponse::Ok().json(ResponseData {
        query: input,
        result,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cache: Cache = Arc::new(DashMap::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cache.clone()))
            .route("/compute", web::get().to(compute_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

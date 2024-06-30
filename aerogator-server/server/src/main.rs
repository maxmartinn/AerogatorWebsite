use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Item {
    id: usize,
    name: String,
}

struct AppState {
    items: Mutex<Vec<Item>>,
}

async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

async fn add_item(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    items.push(item.into_inner());
    HttpResponse::Ok().json("Item added")
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(hello))
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(add_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
struct Info {
    username: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            // Note: using app_data instead of data
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
            .route("/foo", web::post().to(foo))
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// working; test with `curl -X POST http://127.0.0.1:8080/echo -d "user=user1&pass=abcd"`
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// not working?
async fn foo(info: web::Json<Info>) -> Result<String> {
    println!("Welcome {}!", info.username);
    Ok(format!("Welcome {}!", info.username))
}

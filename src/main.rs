use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
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
    let mut counter = data.counter.lock().unwrap();
    *counter += 1; 
    format!("Request number: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
            .route("/foo", web::post().to(foo))
            .service(echo)
            .service(bar)
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

// working;
// On Windows, test with `curl -X POST http://127.0.0.1:8080/foo -d "{\"username\":\"a\"}" -H "Content-Type: application/json"`
async fn foo(info: web::Json<Info>) -> impl Responder {
    println!("Welcome {}!", info.username);
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

// working; 
// On Windows, test with `curl -X GET http://127.0.0.1:8080/bar -d "{\"username\":\"a\"}" -H "Content-Type: application/json"`
#[get("/bar")]
async fn bar(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

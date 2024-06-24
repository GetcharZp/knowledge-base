use actix_web::{App, get, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(ping)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

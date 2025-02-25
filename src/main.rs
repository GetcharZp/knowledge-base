mod router;
mod handler;
mod service;
mod models;
mod dao;
mod define;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    router::run_server().await
}

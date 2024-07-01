use actix_web::{Responder};

#[utoipa::path(
    get,
    path = "/ping",
    tag = "TRY",
)]
pub async fn ping() -> impl Responder {
    "pong"
}

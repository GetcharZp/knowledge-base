use actix_web::{App, HttpServer};
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::handler::admin;
use crate::handler::ping::ping;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "KnowledgeBase",
        version = "24.1.0",
        description = "个人知识库"
    ),
    paths(
        crate::handler::ping::ping,
        crate::handler::admin::user::create,
    ),
    components(schemas(
        crate::handler::admin::user::UserCreateRequest,
    )),
)]
struct ApiDoc;

fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/ping").route(web::get().to(ping)))
        .service(
            web::scope("/api/v1").
                service(
                    web::scope("/admin")
                        .service(web::resource("/user/create").route(web::post().to(admin::user::create)))
                )
        )
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        );
}

pub async fn run_server() ->std::io::Result<()> {
    HttpServer::new(move || {
        App::new().configure(config_app)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

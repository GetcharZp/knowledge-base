use actix_web::{App, HttpServer};
use actix_web::web;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;
use crate::handler::{admin, user};
use crate::handler::ping::ping;
use crate::middleware;


#[derive(OpenApi)]
#[openapi(
    info(
        title = "KnowledgeBase",
        version = "24.1.0",
        description = "个人知识库"
    ),
    paths(
        crate::handler::ping::ping,
        crate::handler::user::login,
        crate::handler::admin::user::create,
        crate::handler::admin::user::reset_password,
        crate::handler::admin::user::list,
    ),
    components(schemas(
        crate::handler::user::UserLoginRequest,
        crate::handler::admin::user::UserCreateRequest,
        crate::handler::admin::user::UserResetPasswordRequest,
        crate::handler::admin::user::UserListRequest,
        crate::handler::admin::user::UserListReply,
        crate::dao::user_basic_dao::UserBasicDao,
    )),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}

fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/ping").route(web::get().to(ping)))
        .service(
            web::scope("/api/v1").
                service(web::resource("/login").route(web::post().to(user::login))).
                service(
                    web::scope("/admin")
                        .service(web::resource("/user/create").route(web::post().to(admin::user::create)))
                        .service(web::resource("/user/reset/password").route(web::post().to(admin::user::reset_password)))
                        .service(web::resource("/user/list").wrap(middleware::auth::AuthMiddleware).route(web::get().to(admin::user::list)))
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

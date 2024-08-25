use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use crate::service::admin::user::create_service;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserCreateRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 邮箱
    pub email: Option<String>,
}

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/admin/user/create",
    request_body = UserCreateRequest,
    responses(
        (status = 200, description = "创建成功")
    ),
    tag = "超管模块-用户管理"
)]
pub async fn create(req: web::Json<UserCreateRequest>) -> impl Responder {
    let reply = create_service(req.into_inner()).await;
    match reply {
        Ok(_) => HttpResponse::Ok().json(json!({"code": 200, "msg": "创建成功"})),
        Err(err) => HttpResponse::Ok().json(json!({"code": -1, "msg": err.to_string()}))
    }
}

use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use crate::service::user::{login_service, password_modify_service};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserLoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserClaim {
    // 唯一标识
    pub uuid: String,
    // 用户名
    pub username: String,
    // 邮箱地址
    pub email: Option<String>,
    // 过期时间
    pub exp: i64,
}

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/login",
    request_body = UserLoginRequest,
    responses(
        (status = 200, description = "登录成功")
    ),
    tag = "用户模块"
)]
pub async fn login(req: web::Json<UserLoginRequest>) -> impl Responder {
    let reply = login_service(req.into_inner()).await;
    match reply {
        Ok(token) => HttpResponse::Ok().json(json!({"code": 200, "data": {"token": token}})),
        Err(err) => HttpResponse::Ok().json(json!({"code": -1, "msg": err.to_string()}))
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct PasswordModifyRequest {
    /// 旧密码
    pub old_password: String,
    /// 新密码
    pub new_password: String,
}

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/password/modify",
    request_body = PasswordModifyRequest,
    responses(
        (status = 200, description = "修改成功")
    ),
    tag = "用户模块",
    security(("Authorization" = []))
)]
pub async fn password_modify(req: HttpRequest, body: web::Json<PasswordModifyRequest>) -> impl Responder {
    let user_claim = req.extensions().get::<UserClaim>().unwrap().clone();
    let reply = password_modify_service(user_claim, body.into_inner()).await;
    match reply {
        Ok(_) => HttpResponse::Ok().json(json!({"code": 200, "msg": "修改成功"})),
        Err(err) => HttpResponse::Ok().json(json!({"code": -1, "msg": err.to_string()}))
    }
}
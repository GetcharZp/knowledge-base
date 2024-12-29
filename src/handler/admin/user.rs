use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{IntoParams, ToSchema};
use crate::dao::user_basic_dao::UserBasicDao;
use crate::service::admin::user::{create_service, list_service, reset_password_service};

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


#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResetPasswordRequest {
    /// 唯一标识
    pub uuid: String,
    /// 密码
    pub password: String,
}

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/admin/user/reset/password",
    request_body = UserResetPasswordRequest,
    responses(
        (status = 200, description = "重置密码")
    ),
    tag = "超管模块-用户管理"
)]
pub async fn reset_password(req: web::Json<UserResetPasswordRequest>) -> impl Responder {
    let reply = reset_password_service(req.into_inner()).await;
    match reply {
        Ok(_) => HttpResponse::Ok().json(json!({"code": 200, "msg": "重置成功"})),
        Err(err) => HttpResponse::Ok().json(json!({"code": -1, "msg": err.to_string()}))
    }
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct UserListRequest {
    /// 当前页
    pub page: i64,
    /// 每页的数据条数
    pub size: i64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserListReply {
    /// 数据
    pub list: Vec<UserBasicDao>,
    /// 总数
    pub total: i64,
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/admin/user/list",
    params(UserListRequest),
    responses(
        (status = 200, description = "用户列表", body = UserListReply)
    ),
    tag = "超管模块-用户管理",
    security(
        ("Authorization" = [])
    )
)]
pub async fn list(req: web::Query<UserListRequest>) -> impl Responder {
    let reply = list_service(req.into_inner()).await;
    match reply {
        Ok(result) => HttpResponse::Ok().json(json!({"code": 200, "data": result})),
        Err(err) => HttpResponse::Ok().json(json!({"code": -1, "msg": err.to_string()})),
    }
}

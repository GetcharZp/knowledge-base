use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserBasicDao {
    /// 唯一标识
    pub uuid: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 邮箱
    pub email: Option<String>,
    /// 创建时间
    pub create_at: i64,
    /// 更新时间
    pub update_at: i64,
}

pub const USER_BASIC_DAO: &str = "user_basic_dao";

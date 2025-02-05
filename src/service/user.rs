use std::error::Error;
use std::io;
use std::io::ErrorKind;
use chrono::{Duration, Utc};
use elasticsearch::{SearchParts, UpdateParts};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{from_value, json, Value};
use crate::dao::init::es_client;
use crate::dao::user_basic_dao::{USER_BASIC_DAO, UserBasicDao};
use crate::define::JWT_SECRET;
use crate::handler::user::{PasswordModifyRequest, UserClaim, UserLoginRequest};

pub async fn login_service(req: UserLoginRequest) -> Result<String, Box<dyn Error>> {
    // 1. es client
    let client = es_client();

    // 2. 获取 user 信息
    let response = client.search(SearchParts::Index(&[USER_BASIC_DAO]))
        .body(json!({
            "query": {
                "term": {
                    "username.keyword": req.username
                }
            }
        }))
        .send()
        .await;
    if let Err(e) = response {
        return Err(Box::new(e))
    }
    let response = response?;
    let response_body = response.json::<Value>().await?;
    println!("{}", response_body);

    let user_basic = from_value::<UserBasicDao>(response_body["hits"]["hits"][0]["_source"].clone());
    if let Err(_e) = user_basic {
        return Err(Box::new(io::Error::new(ErrorKind::NotFound, "用户名不存在")))
    }

    // 3. 校验密码
    let user_basic = user_basic?;
    if user_basic.password != req.password {
        return Err(Box::new(io::Error::new(ErrorKind::InvalidData, "密码不正确")))
    }

    // 4. 生成 token
    let claim = UserClaim {
        uuid: user_basic.uuid,
        username: user_basic.username,
        email: user_basic.email,
        exp: Utc::now().timestamp_millis() + Duration::days(1).num_milliseconds(),
    };
    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(JWT_SECRET.as_ref()))?;

    Ok(token)
}

pub async fn password_modify_service(user_claim: UserClaim, body: PasswordModifyRequest) -> Result<(), Box<dyn Error>> {
    // 1. es client
    let client = es_client();

    // 2. 获取 user 信息
    let response = client.search(SearchParts::Index(&[USER_BASIC_DAO]))
        .body(json!({
            "query": {
                "term": {
                    "username.keyword": user_claim.username
                }
            }
        }))
        .send()
        .await;
    if let Err(e) = response {
        return Err(Box::new(e))
    }
    let response = response?;
    let response_body = response.json::<Value>().await?;
    println!("{}", response_body);

    let user_basic = from_value::<UserBasicDao>(response_body["hits"]["hits"][0]["_source"].clone());
    if let Err(_e) = user_basic {
        return Err(Box::new(io::Error::new(ErrorKind::NotFound, "用户名不存在")))
    }
    let user_id = response_body["hits"]["hits"][0]["_id"].as_str()
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "用户名不存在"))?;

    // 3. 校验密码
    let user_basic = user_basic?;
    if user_basic.password != body.old_password {
        return Err(Box::new(io::Error::new(ErrorKind::InvalidData, "旧密码不正确")))
    }

    // 4. 更新密码
    client.update(UpdateParts::IndexId(USER_BASIC_DAO, user_id))
        .body(json!({
            "doc": {
                "password": body.new_password
            }
        }))
        .send()
        .await?;
    
    Ok(())
}

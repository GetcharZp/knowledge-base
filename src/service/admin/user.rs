use std::error::Error;
use std::io;
use std::io::ErrorKind;
use elasticsearch::{IndexParts, SearchParts};
use serde_json::{from_value, json, Value};
use uuid::Uuid;
use crate::dao::init::es_client;
use crate::dao::user_basic_dao::{USER_BASIC_DAO, UserBasicDao};
use crate::handler::admin::user::{UserCreateRequest, UserListRequest, UserListReply};

pub async fn create_service(req: UserCreateRequest) -> Result<(), Box<dyn Error>> {
    // 1. es client
    let client = es_client();

    // 2. username 存在
    let response = client.search(SearchParts::Index(&[USER_BASIC_DAO]))
        .body(json!({
            "query": {
                "term": {
                    "username": req.username
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
    let total = response_body["hits"]["total"]["value"].as_i64();
    if let Some(total) = total {
        if total > 0 {
            return Err(Box::new(io::Error::new(ErrorKind::AlreadyExists, "用户名已存在")))
        }
    }

    // 3. save
    let ub = UserBasicDao {
        uuid: Uuid::new_v4().to_string(),
        username: req.username,
        password: req.password,
        email: req.email,
        create_at: chrono::Utc::now().timestamp_millis(),
        update_at: chrono::Utc::now().timestamp_millis(),
    };
    client.index(IndexParts::Index(USER_BASIC_DAO))
        .body(json!(ub))
        .send()
        .await?;

    Ok(())
}

pub async fn list_service(req: UserListRequest) -> Result<UserListReply, Box<dyn Error>> {
    // 1. es client
    let client = es_client();

    // 2. 准备查询条件
    let response = client.search(SearchParts::Index(&[USER_BASIC_DAO]))
        .body(
            json!({
                "size": req.size,
                "from": (req.page - 1) * req.size,
                "sort": [
                    {
                        "create_at": {
                            "order": "desc"
                        }
                    }
                ],
                "query": {
                    "match_all": {}
                }
            })
        )
        .send()
        .await;
    if let Err(e) = response {
        return Err(Box::new(e))
    }
    let response = response?;
    let response_body = response.json::<Value>().await?;
    println!("===> {}", response_body);
    // total
    let total = response_body["hits"]["total"]["value"].as_i64().unwrap();
    // list
    let hits = response_body["hits"]["hits"].as_array().unwrap();

    // 3. 结果处理
    let list = hits.iter()
        .map(|hit| from_value(hit["_source"].clone()).unwrap())
        .collect::<Vec<UserBasicDao>>();

    Ok(UserListReply{
        total,
        list
    })
}

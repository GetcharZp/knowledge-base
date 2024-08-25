use std::error::Error;
use std::io;
use std::io::ErrorKind;
use elasticsearch::{Elasticsearch, IndexParts, SearchParts};
use elasticsearch::http::transport::Transport;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::dao::user_basic_dao::{USER_BASIC_DAO, UserBasicDao};
use crate::handler::admin::user::UserCreateRequest;

pub async fn create_service(req: UserCreateRequest) -> Result<(), Box<dyn Error>> {
    // 1. es client
    let transport = Transport::single_node("http://127.0.0.1:9200")?;
    let client = Elasticsearch::new(transport);

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

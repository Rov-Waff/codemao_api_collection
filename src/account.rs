use user_behavior::dtos::AccountLoginVO;
use reqwest::Client;
use std::{collections::HashMap, time::Duration};

use thiserror::Error;

use crate::BASE_URL;
mod user_behavior;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub token: String,
    pub client: Client,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("请求错误,{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON解析错误")]
    Json(#[from] serde_json::Error),
}

impl Account {
    pub async fn new(username: &str, password: &str) -> Result<Account, Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(2))
            .cookie_store(true)
            .build()?;
        let mut reqbody = HashMap::new();
        reqbody.insert("pid", "65edCTyg");
        reqbody.insert("identity", username);
        reqbody.insert("password", password);
        let token = client
            .post(format!("{}tiger/v3/web/accounts/login", BASE_URL))
            .json(&reqbody)
            .send()
            .await?
            .json::<AccountLoginVO>()
            .await?
            .auth
            .token;

        Ok(Account {
            username: username.to_string(),
            password: password.to_string(),
            token,
            client,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenvy::dotenv;

    use std::env;
    #[tokio::test]
    async fn test_login() {
        dotenv().ok();
        let username = env::var("USERNAME").expect("env USERNAME not found");
        let password = env::var("PASSWORD").expect("env PASSWORD not found");
        let account = Account::new(&username, &password).await.expect("Fail!");
        println!("{:?}", &account);
    }
}

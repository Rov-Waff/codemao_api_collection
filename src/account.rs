use dtos::AccountLoginVO;
use reqwest::Client;
use std::{collections::HashMap, time::Duration};

use thiserror::Error;

use crate::BASE_URL;

mod dtos;
mod user_behavior;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Account {
    username: String,
    password: String,
    token: String,
    client: Client,
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
            .build()
            .expect("Failed to create client");
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
    use log::info;
    use std::env;
    #[tokio::test]
    async fn test_login() {
        dotenv().ok();
        let username = env::var("USERNAME").expect("env USERNAME not found");
        let password = env::var("PASSWORD").expect("env PASSWORD not found");
        let account = Account::new(&username, &password).await.expect("Fail!");
        println!("{:?}", &account);
    }
    use user_behavior::UserBehaviors;
    #[tokio::test]
    async fn test_patch_userinfo() {
        env_logger::init();
        dotenv().ok();
        let username = env::var("USERNAME").expect("env USERNAME not found");
        let password = env::var("PASSWORD").expect("env PASSWORD not found");
        let mut account = Account::new(&username, &password).await.expect("Fail!");
        println!("{:?}", &account);
        account
            .patch_user_detail(Some("xiaole436t"), None, None, None, None, None)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_patch_password() {
        env_logger::init();
        dotenv().ok();
        let username = env::var("USERNAME").expect("env USERNAME not found");
        let password = env::var("PASSWORD").expect("env PASSWORD not found");
        let account = Account::new(&username, &password).await.expect("Fail!");
        println!("{:?}", &account);
        account
            .patch_user_password(&password, "Oe2JhU42")
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_get_message_count() {
        dotenv().ok();
        env_logger::init();
        let username = env::var("USERNAME").expect("env USERNAME not found");
        let password = env::var("PASSWORD").expect("env PASSWORD not found");
        let account = Account::new(&username, &password).await.expect("Fail!");
        let res = account.get_message_count().await;
        match res {
            Ok(res) => {
                res.iter().for_each(|r| info!("{:?}", r));
            }
            Err(err) => match err {
                Error::Reqwest(error) => {
                    eprintln!("REQWEST Error,{:?}", error)
                }
                Error::Json(error) => eprintln!("JSON Error,{:?}", error),
            },
        }
    }
    #[tokio::test]
    async fn test_get_user_detail() {
        dotenv().ok();
        env_logger::init();
        let username = env::var("USERNAME").expect("env USERNAME not found");
        let password = env::var("PASSWORD").expect("env PASSWORD not found");
        let account = Account::new(&username, &password).await.expect("Fail!");
        let res = account.get_user_detail().await.unwrap();
        println!("{:?}", res);
    }
    #[tokio::test]
    async fn test_get_other_user_detail() {
        dotenv().ok();
        env_logger::init();
        let username = env::var("USERNAME").expect("env USERNAME not found");
        let password = env::var("PASSWORD").expect("env PASSWORD not found");
        let account = Account::new(&username, &password).await.expect("Fail!");
        let res = account.get_other_user_detail(2615505).await.unwrap();
        info!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_user_honor() {
        dotenv().ok();
        env_logger::init();
        let username = env::var("USERNAME").expect("env USERNAME not found");
        let password = env::var("PASSWORD").expect("env PASSWORD not found");
        let account = Account::new(&username, &password).await.expect("Fail!");
        let res = account.get_user_honor(2615505).await.unwrap();
        info!("{:?}",res);
    }
}

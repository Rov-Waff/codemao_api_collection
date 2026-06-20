use crate::account::user_behavior;

use super::*;
use dotenvy::dotenv;
use log::info;
use std::env;
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
    info!("{:?}", res);
}

#[tokio::test]
async fn test_get_user_works() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.expect("Fail!");
    let res = account.get_user_works(2615505, 0, 5).await.unwrap();
    info!("{:?}", res);
}
#[tokio::test]
async fn test_get_user_collected_works() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.expect("Fail!");
    let res = account
        .get_user_collected_works(2615505, 1, 5)
        .await
        .unwrap();
    info!("{:?}", res)
}
#[tokio::test]
async fn test_get_user_followers() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.expect("Fail!");
    let res = account.get_user_follower(2615505, 1, 5).await.unwrap();
    info!("{:?}", res)
}

#[tokio::test]
async fn test_get_user_fans() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.expect("Fail!");
    let res = account.get_user_fans(2615505, 1, 5).await.unwrap();
    info!("{:?}", res);
}

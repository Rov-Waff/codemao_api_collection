use std::env;

use dotenvy::dotenv;
use log::info;

use crate::{Account, forum::forum_behavior::ForumBehavior};

#[tokio::test]
async fn test_get_all_board() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account.get_all_board_info().await.unwrap();
    info!("{:?}", res);
}

#[tokio::test]
async fn test_get_notice_board() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account.get_notice_board(Some(1)).await.unwrap();
    info!("LIMIT 4-->{:?}", res);
    let res = account.get_notice_board(None).await.unwrap();
    info!("LIMIT NONE --> {:?}", res);
}

#[tokio::test]
async fn test_get_board_info() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account.get_board_info(6).await.unwrap();
    info!("{:?}", res);
}

#[tokio::test]
async fn test_search_post() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account.search_posts("Rust", None, None).await.unwrap();
    info!("{:?}", res);
}

#[tokio::test]
async fn test_get_post_detail() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account.get_post_detail(1651206).await.unwrap();
    info!("{:?}", res);
}
#[tokio::test]
async fn test_post_a_post() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account.post_a_post("四十二号混凝土拌意大利面", "我认为意大利面就应该拌42号混凝土，因为这个螺丝钉的长度它很容易直接影响到挖掘机的扭矩......" ,
    5,
    None).await.unwrap();
    info!("{:?}", res);
}
#[tokio::test]
async fn test_post_a_reply() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account
        .post_a_reply(
            "你往里砸的时候，会在一瞬间产生大量的高能蛋白，俗称UFO",
            1651216,
        )
        .await
        .unwrap();
    info!("{:?}", res);
}

#[tokio::test]
async fn test_post_a_comment() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account
        .post_a_comment("会严重影响经济的发展", 1898387, None)
        .await
        .unwrap();
    info!("{:?}", res);
}

#[tokio::test]
async fn test_post_a_comment_with_parent() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account
        .post_a_comment(
            "甚至就连太平洋以及充电机都会造成一定的盒污染",
            1898387,
            Some(2335257),
        )
        .await
        .unwrap();
    info!("{:?}", res);
}

#[tokio::test]
async fn test_delete_a_post(){
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    account.delete_a_post(1651216).await.unwrap();
}
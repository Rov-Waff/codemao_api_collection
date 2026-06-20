use std::env;

use crate::{Account, community::{banner_type::ALL, community_behavior::CommunityBehavior}};
use dotenvy::dotenv;
#[tokio::test]
async fn test_sign(){
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    account.signature().await.unwrap();
}
#[tokio::test]
async fn test_banner(){
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account.get_community_banner(ALL).await.unwrap();
    println!("{:?}",res);
}
#[tokio::test]
async fn test_report(){
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    let res = account.get_report_reasons().await.unwrap();
    println!("{:?}",res);
}
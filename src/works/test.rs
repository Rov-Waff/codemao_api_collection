use std::env;

use dotenvy::dotenv;
use log::info;
use crate::{Account, works::work_behavior::WorkBehavior};

#[tokio::test]
async fn test_get_work_info() {
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    info!("{:?}",account.get_work_info(270085474).await.unwrap());
}
#[tokio::test]
async fn test_get_work_tree(){
    dotenv().ok();
    env_logger::init();
    let username = env::var("USERNAME").expect("env USERNAME not found");
    let password = env::var("PASSWORD").expect("env PASSWORD not found");
    let account = Account::new(&username, &password).await.unwrap();
    info!("{:?}",account.get_work_info(270085474).await.unwrap())
}

// 其他方法没有测试，因为劲太大了

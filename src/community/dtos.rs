use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct SimpleWrapper<T>{
    pub items:Vec<T>
}
#[derive(Debug,Deserialize,Serialize)]
pub struct BannerItem{
    pub id:String,
    pub title:String,
    pub target_url:String,
    pub background_url:String,
    pub small_background_url:String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ReasonItem{
    pub id:i32,
    pub content:String
}
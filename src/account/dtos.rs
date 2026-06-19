use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum FieldTypes {
    Int(i64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountLoginVO {
    pub auth: AccountLoginVOAuth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountLoginVOAuth {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct MessageCountVO {
    pub query_type: String,
    pub count: i32,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct UserDetailVO {
    pub id: String,
    pub nickname: String,
    pub avatar_url: String,
    pub email: String,
    pub gold: i32,
    pub qq: String,
    pub real_name: String,
    pub sex: String,
    pub username: String,
    pub voice_forbidden: bool,
    pub birthday: i64,
    pub description: String,
    pub phone_number: String,
    pub create_time: i64,
    pub has_password: bool,
    pub user_type: i32,
    pub show_guide_flag: i32,
    pub has_signed: bool,
    pub has_seen_primary_course: i32,
    pub author_level: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Wrapper<T> {
    pub code: i32,
    pub msg: String,
    pub description: String,
    pub data: T,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserFieldInOtherUserDetailUserInfoField {
    pub id: i32,
    pub nickname: String,
    pub sex: i32,
    pub description: String,
    pub doing: String,
    pub level: i32,
    pub avatar: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct OtherUserDetailUserInfoFieldVO {
    pub user: UserFieldInOtherUserDetailUserInfoField,
    #[allow(nonstandard_style, non_snake_case)]
    pub collectionTimes: i32,
    #[allow(nonstandard_style, non_snake_case)]
    pub forkedTimes: i32,
    #[allow(nonstandard_style, non_snake_case)]
    pub praiseTimes: i32,
    #[allow(nonstandard_style, non_snake_case)]
    pub viewTimes: i32,
}
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct OtherUserDetailVO {
    #[allow(nonstandard_style, non_snake_case)]
    pub userInfo: OtherUserDetailUserInfoFieldVO,
    
}

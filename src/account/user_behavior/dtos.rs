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
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserHonorInfoVO {
    pub user_id: i32,
    pub nickname: String,
    pub avatar_url: String,
    pub user_description: String,
    pub doing: String,
    pub attention_status: bool,
    pub block_total: i32,
    pub re_created_total: i32,
    pub attention_total: i32,
    pub fans_total: i32,
    pub collected_total: i32,
    pub liked_total: i32,
    pub view_times: i32,
    pub author_level: i32,
    pub is_official_certification: i32,
    pub subject_id: i32,
    pub work_shop_name: String,
    pub work_shop_level: i32,
    pub like_score: i32,
    pub collect_score: i32,
    pub fork_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageWrapper<T> {
    pub items: Vec<T>,
    pub offset: i32,
    pub limit: i32,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWorksList {
    pub id: i32,
    #[serde(rename = "type")]
    pub work_type: i32,
    pub work_name: String,
    pub preview: String,
    pub view_times: i32,
    pub collect_times: i32,
    pub liked_times: i32,
    pub parent_id: i32,
    pub fork_enable: bool,
    pub fork_times: i32,
    pub publish_time: i32,
    pub description: String,
    pub role: String,
    pub is_coll_work: bool,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct UserCollectedItems{
    pub id:i32,
    pub name:String,
    pub preview:String,
    pub user_id:i32,
    pub nickname:String,
    pub avatar_url:String,
    pub views_count:i32,
    pub likes_count:i32,
    pub collections_count:i32,
    pub publish_time:i32,
    pub work_type:i32,
    pub description:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserFollowersItems{
    pub id:i32,
    pub nickname:String,
    pub avatar_url:String,
    pub n_works:i32,
    pub total_likes:i32,
    pub is_followed:bool,
    pub description:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct RandomUsername{
    pub nickname:String
}

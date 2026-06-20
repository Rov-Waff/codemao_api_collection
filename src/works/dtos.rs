use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct WorkInfoVO {
    pub id: String,
    pub work_name: String,
    #[serde(rename = "type")]
    pub work_type: String,
    pub ide_type: String,
    pub operation: String,
    pub description: String,
    pub player_url: String,
    pub share_url: String,
    pub n_tree_nodes: i32,
    pub view_times: i32,
    pub collect_times: i32,
    pub share_times: i32,
    pub fork_enable: bool,
    pub preview: String,
    pub bcm_version: String,
    pub screenshot_cover_url: String,
    pub comment_times: i32,
    pub publish_time: i32,
    pub work_label_list: Vec<WorkLabelListFieldInWorkInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfoFieldInWorkInfo {
    pub id: i32,
    pub nickname: String,
    pub avatar: String,
    pub description: String,
    pub author_level: i32,
    pub is_official_certification: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AbilitiesFieldInWorkInfo {
    pub is_collected: bool,
    pub is_praised: bool,
    pub is_owned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkLabelListFieldInWorkInfo {
    pub label_type: String,
    pub label_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportWorkDTO {
    pub report_describe: String,
    pub report_reason: String,
    pub work_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkTreeVO {
    pub id: i32,
    pub name: String,
    pub preview: String,
    pub publish_time: i32,
    pub is_published: i32,
    pub is_deleted: i32,
    pub collection_times: i32,
    pub praise_times: i32,
    pub view_times: i32,
    pub parent_id: Option<i32>,
    pub children: Vec<WorkTreeVO>,
    pub author: AuthorFieldInWorkTree,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorFieldInWorkTree {
    pub user_id: i32,
    pub user_name: String,
    pub nickname: String,
}

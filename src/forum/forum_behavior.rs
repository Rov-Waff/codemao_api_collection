use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardItem {
    pub id: String,
    pub name: String,
    pub icon_url: String,
    pub is_hot: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardInfoVO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_url: String,
    pub is_hot: bool,
    pub n_posts: i32,
    pub n_discussions: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoticeBoardItem {
    pub id: String,
    pub post_id: String,
    pub post_title: String,
    pub ordinal: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPostItem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub n_replies: String,
    pub is_authorized: bool,
    pub is_featured: bool,
    pub is_hotted: bool,
    pub is_pinned: bool,
    pub tutorial_flag: i32,
    pub ask_help_flag: i32,
    pub create_at: i32,
    pub n_comments: i32,
    pub replied_at: i32,
    pub commented_at: i32,
    pub user: UserFieldInSearchPostItem,
    pub reply_user: Option<UserFieldInSearchPostItem>,
    pub comment_user: Option<UserFieldInSearchPostItem>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFieldInSearchPostItem {
    pub id: String,
    pub nickname: String,
    pub avatar_url: String,
    pub subject_id: i32,
    pub work_shop_name: String,
    pub work_shop_level: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostAPostDTO {
    pub title: String,
    pub content: String,
    pub studio_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostAPostVO {
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostDetailVO {
    pub id: String,
    pub title: String,
    pub content: String,
    pub board_id: String,
    pub board_name: String,
    pub update_at: i32,
    pub created_at: i32,
    pub n_views: i32,
    pub n_replies: i32,
    pub n_comments: i32,
    pub is_authorized: bool,
    pub is_featured: bool,
    pub is_hotted: bool,
    pub is_pinned: bool,
    pub tutorial_flag: i32,
    pub ask_help_plag: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostAReplyDTO {
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostAReplyVO {
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostACommentDTO {
    pub content: String,
    pub parent_id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostACommentVO {
    pub id: String,
}

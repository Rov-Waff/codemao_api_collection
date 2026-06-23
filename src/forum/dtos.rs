use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardItem {
    pub id: String,
    pub name: String,
    pub icon_url: String,
    pub is_hot: bool,
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
    pub n_replies: i32,
    pub is_authorized: bool,
    pub is_featured: bool,
    pub is_hotted: bool,
    pub is_pinned: bool,
    pub tutorial_flag: i32,
    pub ask_help_flag: i32,
    pub created_at: i32,
    pub n_comments: i32,
    pub replied_at: i32,
    pub commented_at: i32,
    pub user: UserFieldInSearchPostItem,
    pub reply_user: CommentUserFieldInSearchPostItem,
    pub comment_user: CommentUserFieldInSearchPostItem,
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
#[derive(Debug,Serialize,Deserialize)]
pub struct CommentUserFieldInSearchPostItem{
    pub id :String,
    pub nickname:String,
    pub avatar_url:String,
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
    pub updated_at: i32,
    pub created_at: i32,
    pub n_views: i32,
    pub n_replies: i32,
    pub n_comments: i32,
    pub is_authorized: bool,
    pub is_featured: bool,
    pub is_hotted: bool,
    pub is_pinned: bool,
    pub tutorial_flag: i32,
    pub ask_help_flag: i32
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
    pub parent_id: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostACommentVO {
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportPostDTO{
    pub post_id:String,
    pub description:String,
    pub reason_id:String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepliesVO{
    pub id:String,
    pub user:UserFieldInSearchPostItem,
    pub is_top:bool,
    pub n_likes:i32,
    pub is_liked:bool,
    pub content:String,
    pub n_comments:i32,
    pub updated_at:i32,
    pub created_at:i32,
    
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarliestCommentsFieldInGetRepliesVO{
    pub id:String,
    pub user:UserFieldInSearchPostItem,
    pub n_likes:i32,
    pub is_liked:bool,
    pub content:String,
    pub created_at:i32
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCommentVO{
    id:String,
    user:UserFieldInSearchPostItem,
    n_likes:i32,
    is_liked:bool,
    content:String,
    created_at:i32,
    reply_user:Option<ReplyUserFieldInGetCommentVO>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyUserFieldInGetCommentVO{
    id:String,
    nickname:String
}
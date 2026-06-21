//! # 论坛行为
//!
//! 包含了api.codemao.cn下的和论坛相关的所有论坛相关的API接口
//!
use crate::{
    Account, BASE_URL,
    account::{Error, user_behavior::dtos::PageWrapper},
    community::dtos::SimpleWrapper,
    forum::dtos::{
        BoardInfoVO, BoardItem, NoticeBoardItem, PostACommentDTO, PostACommentVO, PostAPostDTO,
        PostAPostVO, PostAReplyDTO, PostAReplyVO, PostDetailVO, ReportPostDTO, SearchPostItem,
    },
};

pub trait ForumBehavior {
    /// 获取全部板块
    ///
    /// # 参数
    ///
    /// 无需参数
    ///
    /// # 返回
    ///
    /// 如果正常工作，则返回`Vec<codemao_api_collection::forum::dtos::BoardItem>`
    fn get_all_board_info(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<BoardItem>, Error>> + Send;
    /// 获取板块信息
    ///
    /// # 参数
    ///
    /// board_id:i32 板块ID
    ///
    /// # 返回
    ///
    /// 如果正常工作，则返回`codemao_api_collection::forum::dtos::BoardInfoVO`
    fn get_board_info(
        &self,
        board_id: i32,
    ) -> impl std::future::Future<Output = Result<BoardInfoVO, Error>> + Send;
    /// 获取公告板
    ///
    /// # 参数
    ///
    /// limit:Option<i32> 返回最大条数
    ///
    /// # 返回
    ///
    /// 如果正常工作，则返回`codemao_api_collection::community::dtos::NoticeBoardItem<codemao_api_collection::forum::dtos::NoticeBoardItem>`
    fn get_notice_board(
        &self,
        limit: Option<i32>,
    ) -> impl std::future::Future<Output = Result<SimpleWrapper<NoticeBoardItem>, Error>> + Send;
    /// 搜索帖子
    ///
    /// # 参数
    ///
    /// title:&str 帖子标题
    /// page:Option<i32> 页码
    /// limit:Option<i32> 每页条数
    ///
    /// # 返回
    ///
    /// 如果正常工作，则返回`codemao_api_collection::account::user_behavior::dtos<codemao_api_collection::forum::dtos::SearchPostItem>`
    fn search_posts(
        &self,
        title: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> impl std::future::Future<Output = Result<PageWrapper<SearchPostItem>, Error>> + Send;
    /// 发布一个帖子
    /// 
    /// # 参数
    /// 
    /// title: &str 标题
    /// content: &str 内容
    /// board_id: i32 板块ID
    /// studio_id: Option<String> 工作室ID，可选
    /// 
    /// # 返回
    /// 
    /// 如果正常工作，则返回 `codemao_api_collection::forum::dtos::PostAPostVO`
    fn post_a_post(
        &self,
        title: &str,
        content: &str,
        board_id: i32,
        studio_id: Option<String>,
    ) -> impl std::future::Future<Output = Result<PostAPostVO, Error>> + Send;
    /// 删除一个帖子
    /// 
    /// # 参数
    /// 
    /// post_id: i32 帖子ID
    /// 
    /// # 返回
    /// 
    /// 如果正常工作，则返回 `()`
    fn delete_a_post(
        &self,
        post_id: i32,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    /// 举报一个帖子
    /// 
    /// # 参数
    /// 
    /// post_id: i32 帖子ID
    /// description: &str 举报描述
    /// reason_id: i32 举报原因ID
    /// 
    /// # 返回
    /// 
    /// 如果正常工作，则返回 `()`
    fn report_a_post(
        &self,
        post_id: i32,
        description: &str,
        reason_id: i32,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    /// 获取帖子详情
    /// 
    /// # 参数
    /// 
    /// post_id: i32 帖子ID
    /// 
    /// # 返回
    /// 
    /// 如果正常工作，则返回 `codemao_api_collection::forum::dtos::PostDetailVO`
    fn get_post_detail(
        &self,
        post_id: i32,
    ) -> impl std::future::Future<Output = Result<PostDetailVO, Error>> + Send;
    /// 回复一个帖子
    /// 
    /// # 参数
    /// 
    /// content: &str 内容
    /// post_id: i32 帖子ID
    /// 
    /// # 返回
    /// 
    /// 如果正常工作，则返回 `codemao_api_collection::forum::dtos::PostAReplyVO`
    fn post_a_reply(
        &self,
        content: &str,
        post_id: i32,
    ) -> impl std::future::Future<Output = Result<PostAReplyVO, Error>> + Send;
    /// 评论一个回复
    /// 
    /// # 参数
    /// 
    /// content: &str 内容
    /// reply_id: i32 回复ID
    /// parent_id: Option<i32> 父评论ID，可选
    /// 
    /// # 返回
    /// 
    /// 如果正常工作，则返回 `codemao_api_collection::forum::dtos::PostACommentVO`
    fn post_a_comment(
        &self,
        content: &str,
        reply_id: i32,
        parent_id: Option<i32>,
    ) -> impl std::future::Future<Output = Result<PostACommentVO, Error>> + Send;
}

impl ForumBehavior for Account {
    async fn get_all_board_info(&self) -> Result<Vec<BoardItem>, Error> {
        Ok(self
            .client
            .get(format!("{}web/forums/boards/simples/all", BASE_URL))
            .send()
            .await?
            .json::<SimpleWrapper<BoardItem>>()
            .await?
            .items)
    }

    async fn get_board_info(&self, board_id: i32) -> Result<BoardInfoVO, Error> {
        Ok(self
            .client
            .get(format!("{}web/forums/boards/{}", BASE_URL, board_id))
            .send()
            .await?
            .json::<BoardInfoVO>()
            .await?)
    }

    async fn get_notice_board(
        &self,
        limit: Option<i32>,
    ) -> Result<SimpleWrapper<NoticeBoardItem>, Error> {
        match limit {
            Some(l) => Ok(self
                .client
                .get(format!("{}web/forums/notice-boards?limit={}", BASE_URL, l))
                .send()
                .await?
                .json::<SimpleWrapper<NoticeBoardItem>>()
                .await?),
            None => Ok(self
                .client
                .get(format!("{}web/forums/notice-boards", BASE_URL))
                .send()
                .await?
                .json::<SimpleWrapper<NoticeBoardItem>>()
                .await?),
        }
    }

    async fn search_posts(
        &self,
        title: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<PageWrapper<SearchPostItem>, Error> {
        let mut url = format!("{}web/forums/posts/search", BASE_URL);
        let mut params = Vec::new();
        params.push(format!("title={}", title));
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<PageWrapper<SearchPostItem>>()
            .await?)
    }

    async fn post_a_post(
        &self,
        title: &str,
        content: &str,
        board_id: i32,
        studio_id: Option<String>,
    ) -> Result<PostAPostVO, Error> {
        let url = format!("{}web/forums/boards/{}/posts", BASE_URL, board_id);
        Ok(self
            .client
            .post(url)
            .header("Cookie", format!("authorization={}", self.token))
            .json(&PostAPostDTO {
                title: title.to_string(),
                content: content.to_string(),
                studio_id: studio_id,
            })
            .send()
            .await?
            .json::<PostAPostVO>()
            .await?)
    }
    async fn delete_a_post(&self, post_id: i32) -> Result<(), Error> {
        self.client
            .delete(format!("{}web/forums/posts/{}", BASE_URL, post_id))
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?;
        Ok(())
    }

    async fn report_a_post(
        &self,
        post_id: i32,
        description: &str,
        reason_id: i32,
    ) -> Result<(), Error> {
        let dto = ReportPostDTO {
            post_id: post_id.to_string(),
            description: description.to_string(),
            reason_id: reason_id.to_string(),
        };
        self.client
            .post(format!("{}web/reports/posts", BASE_URL))
            .header("Cookie", format!("authorization={}", self.token))
            .json(&dto)
            .send()
            .await?;
        Ok(())
    }

    async fn get_post_detail(&self, post_id: i32) -> Result<PostDetailVO, Error> {
        Ok(self
            .client
            .get(format!("{}web/forums/posts/{}/details", BASE_URL, post_id))
            .send()
            .await?
            .json::<PostDetailVO>()
            .await?)
    }

    async fn post_a_reply(&self, content: &str, post_id: i32) -> Result<PostAReplyVO, Error> {
        Ok(self
            .client
            .post(format!("{}web/forums/posts/{}/replies", BASE_URL, post_id))
            .header("Cookie", format!("authorization={}", self.token))
            .json(&PostAReplyDTO {
                content: content.to_string(),
            })
            .send()
            .await?
            .json::<PostAReplyVO>()
            .await?)
    }

    async fn post_a_comment(
        &self,
        content: &str,
        reply_id: i32,
        parent_id: Option<i32>,
    ) -> Result<PostACommentVO, Error> {
        let dto = PostACommentDTO {
            content: content.to_string(),
            parent_id: parent_id,
        };
        Ok(self
            .client
            .post(format!(
                "{}web/forums/replies/{}/comments",
                BASE_URL, reply_id
            ))
            .json(&dto)
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?
            .json::<PostACommentVO>()
            .await?)
    }
}

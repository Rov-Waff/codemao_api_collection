use crate::{
    Account, account::{Error, user_behavior::dtos::Wrapper}, community::dtos::SimpleWrapper, forum::dtos::{
        BoardInfoVO, BoardItem, NoticeBoardItem, PostACommentVO, PostAPostVO, PostAReplyVO,
        PostDetailVO, SearchPostItem,
    }
};

pub trait ForumBehavior {
    fn get_all_board_info(&self) -> impl std::future::Future<Output = Result<SimpleWrapper<BoardItem>, Error>> + Send;
    fn get_board_info(&self, board_id: i32) -> impl std::future::Future<Output = Result<BoardInfoVO, Error>> + Send;
    fn get_notice_board(&self, limit: Option<i32>)
    -> impl std::future::Future<Output = Result<Wrapper<NoticeBoardItem>, Error>> + Send;
    fn search_posts(
        &self,
        title: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> impl std::future::Future<Output = Result<Wrapper<SearchPostItem>, Error>> + Send;
    fn post_a_post(
        &self,
        title: &str,
        content: &str,
        board_id: i32,
        studio_id: Option<i32>,
    ) -> impl std::future::Future<Output = Result<PostAPostVO, Error>> + Send;
    fn delete_a_post(&self, post_id: i32) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn report_a_post(
        &self,
        post_id: i32,
        description: &str,
        reason_id: i32,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn get_post_detail(&self, post_id: i32) -> impl std::future::Future<Output = Result<PostDetailVO, Error>> + Send;
    fn post_a_reply(&self, content: &str) -> impl std::future::Future<Output = Result<PostAReplyVO, Error>> + Send;
    fn post_a_comment(&self, content: &str, parent_id: i32) -> impl std::future::Future<Output = Result<PostACommentVO, Error>> + Send;
}

impl ForumBehavior for Account {
    async fn get_all_board_info(&self) -> Result<SimpleWrapper<BoardItem>, Error> {
        todo!()
    }

    async fn get_board_info(&self, board_id: i32) ->Result<BoardInfoVO, Error>  {
        todo!()
    }

    async fn get_notice_board(&self, limit: Option<i32>)
    ->  Result<Wrapper<NoticeBoardItem>, Error> {
        todo!()
    }

    async fn search_posts(
        &self,
        title: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Wrapper<SearchPostItem>, Error> {
        todo!()
    }

    async fn post_a_post(
        &self,
        title: &str,
        content: &str,
        board_id: i32,
        studio_id: Option<i32>,
    ) -> Result<PostAPostVO, Error>  {
        todo!()
    }

    async fn delete_a_post(&self, post_id: i32) ->Result<(), Error> {
        todo!()
    }

    async fn report_a_post(
        &self,
        post_id: i32,
        description: &str,
        reason_id: i32,
    ) -> Result<(), Error>  {
        todo!()
    }

    async fn get_post_detail(&self, post_id: i32) -> Result<PostDetailVO, Error> {
        todo!()
    }

    async fn post_a_reply(&self, content: &str) -> Result<PostAReplyVO, Error>  {
        todo!()
    }

    async fn post_a_comment(&self, content: &str, parent_id: i32) ->  Result<PostACommentVO, Error>  {
        todo!()
    }
}
//! 账户用户行为辅助函数。
//!
//! 本模块提供 `UserBehaviors` trait 及其对 `Account` 的实现。
//! 包含用于与远程用户/账户相关接口（资料、密码、关注者、作品等）交互的便捷异步方法。
//!
//! 使用方法：导入该 trait 后即可在 `Account` 实例上调用这些扩展方法。

use log::{debug, info};
use std::collections::HashMap;

use crate::account::user_behavior::dtos::*;
use crate::{
    BASE_URL,
    account::{Account, Error},
};
#[cfg(feature = "python")]
use pyo3::prelude::*;
pub mod dtos;
#[allow(dead_code)]
/// 账户相关用户行为扩展方法。
///
/// 方法为异步并返回 `Result<..., Error>`。已在本模块为 `Account` 实现，
/// 导入该 trait 后即可在 `Account` 上直接调用这些辅助方法。
pub trait UserBehaviors {
    fn patch_user_detail(
        &mut self,
        nickname: Option<&str>,
        fullname: Option<&str>,
        description: Option<&str>,
        sex: Option<i8>,
        birthday: Option<i64>,
        avatar_url: Option<&str>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    /// 修改当前账户密码。
    ///
    /// `old_password` 为旧密码，`password` 为新密码。
    fn patch_user_password(
        &self,
        old_password: &str,
        password: &str,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn get_message_count(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<MessageCountVO>, Error>> + Send;
    /// 获取当前登录用户的详细信息。
    fn get_user_detail(
        &self,
    ) -> impl std::future::Future<Output = Result<UserDetailVO, Error>> + Send;
    fn update_token(&mut self) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    /// 根据用户 id 获取他人的公开资料。
    fn get_other_user_detail(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = Result<OtherUserDetailVO, Error>> + Send;
    /// 获取用户荣誉/勋章信息。
    fn get_user_honor(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = Result<UserHonorInfoVO, Error>> + Send;
    /// 列出用户创建的作品。
    fn get_user_works(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserWorksList>, Error>> + Send;
    /// 列出用户收藏的作品。
    fn get_user_collected_works(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserCollectedItems>, Error>> + Send;
    /// 列出指定用户关注的人。
    fn get_user_follower(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserFollowersItems>, Error>> + Send;
    /// 列出指定用户的粉丝（关注者）。
    fn get_user_fans(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserFollowersItems>, Error>> + Send;
    /// 关注某个用户（当前登录账户关注 `user_id`）。
    fn follow_user(
        &self,
        user_id: i32,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    /// 从服务端请求一个可用的随机用户名。
    fn get_random_username(
        &self,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
}

impl UserBehaviors for Account {
    async fn patch_user_detail(
        &mut self,
        nickname: Option<&str>,
        fullname: Option<&str>,
        description: Option<&str>,
        sex: Option<i8>,
        birthday: Option<i64>,
        avatar_url: Option<&str>,
    ) -> Result<(), Error> {
        let mut body = HashMap::new();

        if let Some(v) = nickname {
            body.insert("nickname", FieldTypes::String(v.to_string()));
        }
        if let Some(v) = fullname {
            body.insert("fullname", FieldTypes::String(v.to_string()));
        }
        if let Some(v) = description {
            body.insert("description", FieldTypes::String(v.to_string()));
        }
        if let Some(v) = sex {
            body.insert("sex", FieldTypes::Int(v as i64));
        }
        if let Some(v) = birthday {
            body.insert("birthday", FieldTypes::Int(v));
        }
        if let Some(v) = avatar_url {
            body.insert("avatar_url", FieldTypes::String(v.to_string()));
        }

        debug!(
            "patch_user_detail payload: nickname={:?}, fullname={:?}, sex={:?}, birthday={:?}, avatar_url={:?}",
            nickname, fullname, sex, birthday, avatar_url
        );
        debug!("patch_user_detail request body: {:?}", body);
        let req = self
            .client
            .patch(format!("{}tiger/v3/web/accounts/info", BASE_URL))
            .json(&body)
            .header("Cookie", format!("authorization={}", self.token))
            .build()?;
        debug!("patch_user_detail request headers: {:?}", req.headers());
        let resp = self.client.execute(req).await?.error_for_status()?;

        info!(
            "patch_user_detail response status: {}, headers: {:?}",
            resp.status(),
            resp.headers()
        );

        Ok(())
    }

    async fn patch_user_password(&self, old_password: &str, password: &str) -> Result<(), Error> {
        let mut json_body = HashMap::new();
        json_body.insert("old_password", old_password);
        json_body.insert("password", password);
        json_body.insert("confirm_password", password);
        let text = self
            .client
            .patch(format!("{}tiger/v3/web/accounts/password", BASE_URL))
            .header("Cookie", format!("authorization={}", self.token))
            .json(&json_body)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        debug!("Patched user password TEXT :{}", text);
        Ok(())
    }

    async fn get_message_count(&self) -> Result<Vec<MessageCountVO>, Error> {
        let resp = self
            .client
            .get(format!("{}web/message-record/count", BASE_URL))
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?
            .error_for_status()?;
        let mess = resp.json::<Vec<MessageCountVO>>().await?;
        Ok(mess)
    }

    async fn get_user_detail(&self) -> Result<UserDetailVO, Error> {
        let resp = self
            .client
            .get(format!("{}web/users/details", BASE_URL))
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?
            .error_for_status()?;
        let res = resp.json::<UserDetailVO>().await?;
        info!("Get user detail successful! CONTENT:{:?}", res);
        Ok(res)
    }

    async fn update_token(&mut self) -> Result<(), Error> {
        let mut reqbody = HashMap::new();
        reqbody.insert("pid", "65edCTyg");
        reqbody.insert("identity", &self.username);
        reqbody.insert("password", &self.password);
        let token = self
            .client
            .post(format!("{}tiger/v3/web/accounts/login", BASE_URL))
            .json(&reqbody)
            .send()
            .await?
            .error_for_status()?
            .json::<AccountLoginVO>()
            .await?
            .auth
            .token;
        self.token = token;
        Ok(())
    }

    async fn get_other_user_detail(&self, id: i32) -> Result<OtherUserDetailVO, Error> {
        let resp = self
            .client
            .get(format!("{}api/user/info/detail/{}", BASE_URL, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json::<Wrapper<OtherUserDetailVO>>().await?.data)
    }

    async fn get_user_honor(&self, id: i32) -> Result<UserHonorInfoVO, Error> {
        let resp = self
            .client
            .get(format!(
                "{}creation-tools/v1/user/center/honor?user_id={}",
                BASE_URL, id
            ))
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?
            .error_for_status()?;
        let res = resp.json::<UserHonorInfoVO>().await?;
        Ok(res)
    }

    async fn get_user_works(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> Result<PageWrapper<UserWorksList>, Error> {
        let resp = self
            .client
            .get(format!(
                "{}creation-tools/v1/user/center/work-list?user_id={}&offset={}&limit={}",
                BASE_URL, user_id, offset, limit
            ))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json::<PageWrapper<UserWorksList>>().await?)
    }

    async fn get_user_collected_works(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> Result<PageWrapper<UserCollectedItems>, Error> {
        let resp = self
            .client
            .get(format!(
                "{}creation-tools/v1/user/center/collect/list?user_id={}&offset={}&limit={}",
                BASE_URL, user_id, offset, limit
            ))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json::<PageWrapper<UserCollectedItems>>().await?)
    }

    async fn get_user_follower(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> Result<PageWrapper<UserFollowersItems>, Error> {
        let resp = self
            .client
            .get(format!(
                "{}creation-tools/v1/user/followers?user_id={}&offset={}&limit={}",
                BASE_URL, user_id, offset, limit
            ))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json::<PageWrapper<UserFollowersItems>>().await?)
    }

    async fn get_user_fans(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> Result<PageWrapper<UserFollowersItems>, Error> {
        let resp = self
            .client
            .get(format!(
                "{}creation-tools/v1/user/fans?user_id={}&offset={}&limit={}",
                BASE_URL, user_id, offset, limit
            ))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json::<PageWrapper<UserFollowersItems>>().await?)
    }

    async fn follow_user(&self, user_id: i32) -> Result<(), Error> {
        self.client
            .post(format!("{}nemo/v2/user/{}/follow", BASE_URL, user_id))
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn get_random_username(&self) -> Result<String, Error> {
        let resp = self
            .client
            .get(format!("{}api/user/random/nickname", BASE_URL))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json::<Wrapper<RandomUsername>>().await?.data.nickname)
    }
}

#[cfg(test)]
mod user_behavior_test;

#[cfg(feature = "python")]
#[pymethods]
impl Account {
    fn patch_user_detail(
        &mut self,
        nickname: Option<String>,
        fullname: Option<String>,
        description: Option<String>,
        sex: Option<i32>,
        birthday: Option<i64>,
        avatar_url: Option<String>,
    ) -> PyResult<()> {
        crate::account::get_runtime().block_on(UserBehaviors::patch_user_detail(
            self,
            nickname.as_deref(),
            fullname.as_deref(),
            description.as_deref(),
            sex.map(|s| s as i8),
            birthday,
            avatar_url.as_deref(),
        ))?;
        Ok(())
    }

    fn patch_user_password(&self, old_password: &str, password: &str) -> PyResult<()> {
        crate::account::get_runtime().block_on(
            UserBehaviors::patch_user_password(self, old_password, password)
        )?;
        Ok(())
    }

    fn get_message_count<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            UserBehaviors::get_message_count(self)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn get_user_detail<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            UserBehaviors::get_user_detail(self)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn update_token(&mut self) -> PyResult<()> {
        crate::account::get_runtime().block_on(UserBehaviors::update_token(self))?;
        Ok(())
    }

    fn get_other_user_detail<'py>(&self, id: i32, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            UserBehaviors::get_other_user_detail(self, id)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn get_user_honor<'py>(&self, id: i32, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            UserBehaviors::get_user_honor(self, id)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn get_user_works<'py>(&self, user_id: i32, offset: i32, limit: i32, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            UserBehaviors::get_user_works(self, user_id, offset, limit)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn get_user_collected_works<'py>(&self, user_id: i32, offset: i32, limit: i32, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            UserBehaviors::get_user_collected_works(self, user_id, offset, limit)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn get_user_follower<'py>(&self, user_id: i32, offset: i32, limit: i32, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            UserBehaviors::get_user_follower(self, user_id, offset, limit)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn get_user_fans<'py>(&self, user_id: i32, offset: i32, limit: i32, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            UserBehaviors::get_user_fans(self, user_id, offset, limit)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn follow_user(&self, user_id: i32) -> PyResult<()> {
        crate::account::get_runtime().block_on(UserBehaviors::follow_user(self, user_id))?;
        Ok(())
    }

    fn get_random_username(&self) -> PyResult<String> {
        crate::account::get_runtime().block_on(UserBehaviors::get_random_username(self))
            .map_err(Into::into)
    }
}

use log::{debug, info};
use std::collections::HashMap;

use crate::account::user_behavior::dtos::*;
use crate::{
    BASE_URL,
    account::{Account, Error},
};
pub mod dtos;
#[allow(dead_code)]
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
    fn patch_user_password(&self, old_password: &str, password: &str) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn get_message_count(&self) -> impl std::future::Future<Output = Result<Vec<MessageCountVO>, Error>> + Send;
    fn get_user_detail(&self) -> impl std::future::Future<Output = Result<UserDetailVO, Error>> + Send;
    fn update_token(&mut self) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn get_other_user_detail(&self, id: i32) -> impl std::future::Future<Output = Result<OtherUserDetailVO, Error>> + Send;
    fn get_user_honor(&self, id: i32) -> impl std::future::Future<Output = Result<UserHonorInfoVO, Error>> + Send;
    fn get_user_works(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserWorksList>, Error>> + Send;
    fn get_user_collected_works(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserCollectedItems>, Error>> + Send;
    fn get_user_follower(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserFollowersItems>, Error>> + Send;
    fn get_user_fans(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserFollowersItems>, Error>> + Send;
    fn follow_user(&self, user_id: i32) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn get_random_username(&self) -> impl std::future::Future<Output = Result<String, Error>> + Send;
}

impl UserBehaviors for Account {
    fn patch_user_detail(
        &mut self,
        nickname: Option<&str>,
        fullname: Option<&str>,
        description: Option<&str>,
        sex: Option<i8>,
        birthday: Option<i64>,
        avatar_url: Option<&str>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send {
        async move {
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
            let resp = self.client.execute(req).await?;

            info!(
                "patch_user_detail response status: {}, headers: {:?}",
                resp.status(),
                resp.headers()
            );

            Ok(())
        }
    }

    fn patch_user_password(&self, old_password: &str, password: &str) -> impl std::future::Future<Output = Result<(), Error>> + Send {
        async move {
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
                .text()
                .await?;
            dbg!("Patched user password TEXT :{}", text);
            Ok(())
        }
    }

    fn get_message_count(&self) -> impl std::future::Future<Output = Result<Vec<MessageCountVO>, Error>> + Send {
        async move {
            let mess = self
                .client
                .get(format!("{}web/message-record/count", BASE_URL))
                .header("Cookie", format!("authorization={}", self.token))
                .send()
                .await?
                .json::<Vec<MessageCountVO>>()
                .await?;
            Ok(mess)
        }
    }

    fn get_user_detail(&self) -> impl std::future::Future<Output = Result<UserDetailVO, Error>> + Send {
        async move {
            let res = self
                .client
                .get(format!("{}web/users/details", BASE_URL))
                .header("Cookie", format!("authorization={}", self.token))
                .send()
                .await?
                .json::<UserDetailVO>()
                .await?;
            info!("Get user detail successful! CONTENT:{:?}", res);
            Ok(res)
        }
    }

    fn update_token(&mut self) -> impl std::future::Future<Output = Result<(), Error>> + Send {
        async move {
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
                .json::<AccountLoginVO>()
                .await?
                .auth
                .token;
            self.token = token;
            Ok(())
        }
    }

    fn get_other_user_detail(&self, id: i32) -> impl std::future::Future<Output = Result<OtherUserDetailVO, Error>> + Send {
        async move {
            Ok(self
                .client
                .get(format!("{}api/user/info/detail/{}", BASE_URL, id))
                .send()
                .await?
                .json::<Wrapper<OtherUserDetailVO>>()
                .await?
                .data)
        }
    }

    fn get_user_honor(&self, id: i32) -> impl std::future::Future<Output = Result<UserHonorInfoVO, Error>> + Send {
        async move {
            let res = self
                .client
                .get(format!(
                    "{}creation-tools/v1/user/center/honor?user_id={}",
                    BASE_URL, id
                ))
                .header("Cookie", format!("authorization={}", self.token))
                .send()
                .await?
                .json::<UserHonorInfoVO>()
                .await?;
            Ok(res)
        }
    }

    fn get_user_works(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserWorksList>, Error>> + Send {
        async move {
            Ok(self
                .client
                .get(format!(
                    "{}creation-tools/v1/user/center/work-list?user_id={}&offset={}&limit={}",
                    BASE_URL, user_id, offset, limit
                ))
                .send()
                .await?
                .json::<PageWrapper<UserWorksList>>()
                .await?)
        }
    }

    fn get_user_collected_works(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserCollectedItems>, Error>> + Send {
        async move {
            let res = self
                .client
                .get(format!(
                    "{}creation-tools/v1/user/center/collect/list?user_id={}&offset={}&limit={}",
                    BASE_URL, user_id, offset, limit
                ))
                .send()
                .await?
                .json::<PageWrapper<UserCollectedItems>>()
                .await?;
            Ok(res)
        }
    }

    fn get_user_follower(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserFollowersItems>, Error>> + Send {
        async move {
            let res = self
                .client
                .get(format!(
                    "{}creation-tools/v1/user/followers?user_id={}&offset={}&limit={}",
                    BASE_URL, user_id, offset, limit
                ))
                .send()
                .await?
                .json::<PageWrapper<UserFollowersItems>>()
                .await?;
            Ok(res)
        }
    }

    fn get_user_fans(
        &self,
        user_id: i32,
        offset: i32,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<PageWrapper<UserFollowersItems>, Error>> + Send {
        async move {
            let res = self
                .client
                .get(format!(
                    "{}creation-tools/v1/user/fans?user_id={}&offset={}&limit={}",
                    BASE_URL, user_id, offset, limit
                ))
                .send()
                .await?
                .json::<PageWrapper<UserFollowersItems>>()
                .await?;
            Ok(res)
        }
    }

    fn follow_user(&self, user_id: i32) -> impl std::future::Future<Output = Result<(), Error>> + Send {
        async move {
            self.client
                .post(format!("{}nemo/v2/user/{}/follow", BASE_URL, user_id))
                .header("Cookie", format!("authorization={}", self.token))
                .send()
                .await?;
            Ok(())
        }
    }

    fn get_random_username(&self) -> impl std::future::Future<Output = Result<String, Error>> + Send {
        async move {
            Ok(self
                .client
                .get(format!("{}api/user/random/nickname", BASE_URL))
                .send()
                .await?
                .json::<Wrapper<RandomUsername>>()
                .await?
                .data
                .nickname)
        }
    }
}

#[cfg(test)]
mod user_behavior_test;

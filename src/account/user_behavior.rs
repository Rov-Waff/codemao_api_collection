use log::{debug, info};
use std::collections::HashMap;

use crate::{
    BASE_URL,
    account::{
        Account, Error,
        dtos::{FieldTypes, MessageCountVO, UserDetailVO},
    },
};

pub trait UserBehaviors {
    async fn patch_user_detail(
        &self,
        nickname: Option<&str>,
        fullname: Option<&str>,
        description: Option<&str>,
        sex: Option<i8>,
        birthday: Option<i64>,
        avatar_url: Option<&str>,
    ) -> Result<(), Error>;
    async fn patch_user_password(&self, old_password: &str, password: &str) -> Result<(), Error>;
    async fn get_message_count(&self) -> Result<MessageCountVO, Error>;
    async fn get_user_detail(&self) -> Result<UserDetailVO, Error>;
}

impl UserBehaviors for Account {
    async fn patch_user_detail(
        &self,
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
        let resp = self.client.execute(req).await?;

        info!(
            "patch_user_detail response status: {}, headers: {:?}",
            resp.status(),
            resp.headers()
        );
        Ok(())
    }

    async fn patch_user_password(&self, old_password: &str, password: &str) -> Result<(), Error> {
        todo!()
    }

    async fn get_message_count(&self) -> Result<MessageCountVO, Error> {
        todo!()
    }

    async fn get_user_detail(&self) -> Result<UserDetailVO, Error> {
        todo!()
    }
}

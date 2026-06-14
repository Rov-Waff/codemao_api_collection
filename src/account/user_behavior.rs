use log::{debug, info};
use std::collections::HashMap;

use crate::{
    BASE_URL,
    account::{
        Account, Error,
        dtos::{AccountLoginVO, FieldTypes, MessageCountVO, UserDetailVO},
    },
};

pub trait UserBehaviors {
    async fn patch_user_detail(
        &mut self,
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
    async fn update_token(&mut self) -> Result<(), Error>;
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
        let resp = self.client.execute(req).await?;

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
        self.client
            .patch(format!("{}tiger/v3/web/accounts/password", BASE_URL))
            .json(&json_body)
            .send()
            .await?;
        Ok(())
    }

    async fn get_message_count(&self) -> Result<MessageCountVO, Error> {
        todo!()
    }

    async fn get_user_detail(&self) -> Result<UserDetailVO, Error> {
        todo!()
    }

    async fn update_token(&mut self) -> Result<(), Error> {
        //刷新Token
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

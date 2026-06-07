use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct AccountLoginVO {
    pub(super) auth: AccountLoginVOAuth,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct AccountLoginVOAuth {
    pub(super) token: String,
}


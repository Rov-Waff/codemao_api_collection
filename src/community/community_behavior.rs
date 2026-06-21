//! 与社区相关的行为和 Account 的便捷方法。
//!
//! 本模块导出 `CommunityBehavior` trait，提供异步辅助方法以便与社区相关的
//! 接口交互，例如发送用户等级签名、获取社区横幅和检索举报理由等。

use crate::{
    Account, BASE_URL,
    account::Error,
    community::{
        banner_type::ALL,
        dtos::{BannerItem, ReasonItem, SimpleWrapper},
    },
};
#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Trait that provides community-related API actions for an Account.
///
/// Implementors (currently `Account`) gain async methods to interact with
/// community endpoints. These methods return futures that resolve to the
/// corresponding result types.
pub trait CommunityBehavior {
    /// 为当前用户发送等级签名请求。
    ///
    /// 该方法向 nemo 签名端点发送 POST 请求，成功时返回 Ok(())，失败时返回 `Error`。
    fn signature(&self) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    /// 获取社区横幅，按 `banner_type` 过滤。
    ///
    /// 若 `banner_type` 等于常量 `ALL`，返回所有横幅。否则使用该类型作为查询参数。
    /// 成功时返回 `BannerItem` 向量。
    fn get_community_banner(&self, banner_type: &str) -> impl std::future::Future<Output = Result<Vec<BannerItem>, Error>> + Send;

    /// 获取举报理由列表。
    ///
    /// 成功时返回 `ReasonItem` 向量。
    fn get_report_reasons(&self) -> impl std::future::Future<Output = Result<Vec<ReasonItem>, Error>> + Send;
}

impl CommunityBehavior for Account {
    async fn signature(&self) -> Result<(), Error> {
        self.client
            .post(format!("{}nemo/v3/user/level/signature", BASE_URL))
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?;
        Ok(())
    }

    async fn get_community_banner(&self, banner_type: &str) -> Result<Vec<BannerItem>, Error> {
        if banner_type == ALL {
            Ok(self
                .client
                .get(format!("{}web/banners/all", BASE_URL))
                .send()
                .await?
                .json::<SimpleWrapper<BannerItem>>()
                .await?
                .items)
        } else {
            Ok(self
                .client
                .get(format!("{}web/banners/all?type={}", BASE_URL, banner_type))
                .send()
                .await?
                .json::<SimpleWrapper<BannerItem>>()
                .await?
                .items)
        }
    }

    async fn get_report_reasons(&self) -> Result<Vec<ReasonItem>, Error> {
        Ok(self
            .client
            .get(format!("{}web/reports/reasons/all",BASE_URL))
            .send()
            .await?
            .json::<SimpleWrapper<ReasonItem>>()
            .await?
            .items)
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl Account {
    fn signature(&self) -> PyResult<()> {
        crate::account::get_runtime().block_on(CommunityBehavior::signature(self))?;
        Ok(())
    }

    fn get_community_banner<'py>(&self, banner_type: &str, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            CommunityBehavior::get_community_banner(self, banner_type)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }

    fn get_report_reasons<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let result = crate::account::get_runtime().block_on(
            CommunityBehavior::get_report_reasons(self)
        )?;
        crate::python_bindings::to_pyobject(&result, py)
    }
}

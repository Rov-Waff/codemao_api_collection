//!# 作品行为
//!
//! 包含了api.codemao.cn下的和作品相关的用户行为
//!

use crate::{
    Account, BASE_URL,
    account::Error,
    works::dtos::{ReportWorkDTO, WorkInfoVO, WorkTreeVO},
};

pub trait WorkBehavior {
    /// 取消发布一个已经发布的作品
    ///
    /// # 参数
    ///
    /// - work_id:i32 需要取消发布作品的ID
    ///
    fn unpublish_work(
        &self,
        work_id: i32,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    /// 获取一个作品的信息
    ///
    /// # 参数
    ///
    /// - work_id:i32 需要获取信息的作品的ID
    ///
    /// # 返回
    ///
    /// 如果正常工作，返回的类型为`codemao_api_collection::works::dtos::WorkInfoVO`
    fn get_work_info(
        &self,
        work_id: i32,
    ) -> impl std::future::Future<Output = Result<WorkInfoVO, Error>> + Send;
    /// 举报一个作品，劲大，慎用！
    ///
    /// # 参数
    ///
    /// - report_describe:String 举报描述
    /// - report_reason:String 举报理由
    ///
    /// # 返回
    ///
    /// 如果正常工作，则不返回任何内容
    fn report_work(
        &self,
        report_describe: String,
        report_reasons: String,
        work_id: i32,
    ) -> impl Future<Output = Result<(), Error>> + Send;
    /// 获取一个作品再创作树
    ///
    /// # 参数
    ///
    /// work_id:i32 作品ID
    ///
    /// # 返回
    ///
    /// 如果正常工作，则返回`codemao_api_collection::works::dtos::WorkTreeVO`
    fn get_work_tree(&self, work_id: i32) -> impl Future<Output = Result<WorkTreeVO, Error>> + Send;
}

impl WorkBehavior for Account {
    async fn unpublish_work(&self, work_id: i32) -> Result<(), Error> {
        self.client
            .patch(format!("{}tiger/work/{}/unpublish", BASE_URL, work_id))
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?;
        Ok(())
    }

    async fn get_work_info(&self, work_id: i32) -> Result<WorkInfoVO, Error> {
        Ok(self
            .client
            .get(format!("{}creation-tools/v1/works/{}", BASE_URL, work_id))
            .send()
            .await?
            .json::<WorkInfoVO>()
            .await?)
    }

    async fn report_work(
        &self,
        report_describe: String,
        report_reasons: String,
        work_id: i32,
    ) -> Result<(), Error> {
        let dto = ReportWorkDTO {
            report_describe: report_describe,
            report_reason: report_reasons,
            work_id: work_id,
        };
        self.client
            .post(format!("{}nemo/v2/report/work", BASE_URL))
            .json(&dto)
            .header("Cookie", format!("authorization={}", self.token))
            .send()
            .await?;
        Ok(())
    }

    async fn get_work_tree(&self, work_id: i32) -> Result<WorkTreeVO, Error> {
        Ok(self
            .client
            .get(format!("{}tiger/work/tree/{}", BASE_URL, work_id))
            .send()
            .await?
            .json::<WorkTreeVO>()
            .await?)
    }
}

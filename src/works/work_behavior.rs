use crate::{
    Account, BASE_URL,
    account::Error,
    works::dtos::{ReportWorkDTO, WorkInfoVO, WorkTreeVO},
};

pub trait WorkBehavior {
    fn unpublish_work(
        &self,
        work_id: i32,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn get_work_info(
        &self,
        work_id: i32,
    ) -> impl std::future::Future<Output = Result<WorkInfoVO, Error>> + Send;
    fn report_work(
        &self,
        report_describe: String,
        report_reasons: String,
        work_id: i32,
    ) -> impl Future<Output = Result<(), Error>> + Send;
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

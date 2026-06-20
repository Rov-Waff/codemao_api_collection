use crate::{
    Account, BASE_URL,
    account::Error,
    community::{
        banner_type::ALL,
        dtos::{BannerItem, ReasonItem, SimpleWrapper},
    },
};

trait CommunityBehavior {
    async fn signature(&self) -> Result<(), Error>;
    async fn get_community_banner(&self, banner_type: &str) -> Result<Vec<BannerItem>, Error>;
    async fn get_report_reasons(&self) -> Result<Vec<ReasonItem>, Error>;
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
                .post(format!("{}web/banners/all", BASE_URL))
                .send()
                .await?
                .json::<SimpleWrapper<BannerItem>>()
                .await?
                .items)
        } else {
            Ok(self
                .client
                .post(format!("{}web/banners/all?type={}", BASE_URL, banner_type))
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

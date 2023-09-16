use anyhow::Context;

use crate::aws_regional_product_services::AwsRegionalProductServices;

const URL: &str = "https://api.regional-table.region-services.aws.a2z.com/index.json";

pub async fn fetch() -> Result<AwsRegionalProductServices, reqwest::Error> {
    reqwest::get(URL)
        .await
        .with_context(|| format!("Failed to fetch {}", URL))
        .unwrap()
        .json()
        .await
}

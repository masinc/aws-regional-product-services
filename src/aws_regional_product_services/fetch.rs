use crate::aws_regional_product_services::AwsRegionalProductServices;

const URL: &str = "https://api.regional-table.region-services.aws.a2z.com/index.json";

pub async fn fetch() -> Result<AwsRegionalProductServices, reqwest::Error> {
    reqwest::get(URL).await?.json().await
}

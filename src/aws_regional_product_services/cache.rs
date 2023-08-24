use crate::aws_regional_product_services::AwsRegionalProductServices;
use crate::config::get_data_json_path;

#[derive(Debug)]
pub struct Cache {
    data: Option<AwsRegionalProductServices>,
}

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        let p = get_data_json_path();

        if p.exists() {
            let data = std::fs::read_to_string(p)?;
            let data = serde_json::from_str::<AwsRegionalProductServices>(&data)?;
            Ok(Self { data: Some(data) })
        } else {
            Ok(Self { data: None })
        }
    }

    pub fn get(&self) -> Option<&AwsRegionalProductServices> {
        self.data.as_ref()
    }
}

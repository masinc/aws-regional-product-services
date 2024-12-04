use std::io::Read;

use anyhow::Context;

use crate::aws_regional_product_services::AwsRegionalProductServices;
use crate::config::get_data_json_path;

use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct Cache {
    modified_time: Option<chrono::DateTime<chrono::Utc>>,
    json: Option<AwsRegionalProductServices>,
}

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        let p = get_data_json_path();

        if let Ok(file) = std::fs::File::open(&p) {
            let json = Self::get_json(&file)?;
            let modified_time = Self::get_modified_time(&file)?;

            Ok(Self {
                json: Some(json),
                modified_time: Some(modified_time),
            })
        } else {
            Ok(Self {
                json: None,
                modified_time: None,
            })
        }
    }

    fn get_modified_time(file: &std::fs::File) -> anyhow::Result<DateTime<Utc>> {
        let metadata = file.metadata().with_context(|| "Failed to get metadata")?;
        let modified_time = metadata
            .modified()
            .with_context(|| "Failed to get modified time")?;
        Ok(DateTime::from(modified_time))
    }

    fn get_json(file: &std::fs::File) -> anyhow::Result<AwsRegionalProductServices> {
        let mut data = String::new();

        std::io::BufReader::new(file)
            .read_to_string(&mut data)
            .with_context(|| "Failed to read data")?;

        serde_json::from_str::<AwsRegionalProductServices>(&data)
            .with_context(|| "Failed to parse data")
    }

    pub fn get(&self) -> Option<&AwsRegionalProductServices> {
        // expire cache after 1 day

        if let Some(modified_time) = self.modified_time {
            if modified_time + Duration::days(1) < Utc::now() {
                return None;
            }
        }

        self.json.as_ref()
    }
}

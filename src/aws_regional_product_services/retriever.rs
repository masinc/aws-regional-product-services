use crate::aws_regional_product_services;
use crate::aws_regional_product_services::{AwsRegionalProductServices, Cache};
use crate::config::Config;
use crate::config::{get_data_json_path, FetchMode};

#[derive(Debug)]
pub struct Retriever {
    cache: Cache,
}

impl Retriever {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            cache: Cache::new().unwrap(),
        })
    }

    fn save_data_json(&self, data: &AwsRegionalProductServices) -> anyhow::Result<()> {
        let p = get_data_json_path();
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let data = serde_json::to_string_pretty(data)?;
        std::fs::write(p, data)?;
        Ok(())
    }

    pub async fn retrieve(&self) -> anyhow::Result<AwsRegionalProductServices> {
        self.retrieve_with(RetrieveMode::FetchOrCache).await
    }

    pub async fn retrieve_with(
        &self,
        mode: RetrieveMode,
    ) -> anyhow::Result<AwsRegionalProductServices> {
        match mode {
            RetrieveMode::FetchOrCache => self.retrieve_fetch_or_cache().await,
            RetrieveMode::Fetch => self.retrieve_fetch().await,
        }
    }

    async fn retrieve_fetch(&self) -> anyhow::Result<AwsRegionalProductServices> {
        let data = aws_regional_product_services::fetch().await?;
        self.save_data_json(&data)?;
        Ok(data)
    }

    async fn retrieve_fetch_or_cache(&self) -> anyhow::Result<AwsRegionalProductServices> {
        let config = Config::create_or_load_default_path()?;

        match config.fetch_mode {
            FetchMode::Default => {
                if let Some(data) = self.cache.get() {
                    Ok(data.clone())
                } else {
                    let data = self.retrieve_fetch().await?;
                    Ok(data)
                }
            }
            FetchMode::Always => {
                let data = self.retrieve_fetch().await?;
                Ok(data)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetrieveMode {
    FetchOrCache,
    Fetch,
}

use anyhow::Context;

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
            cache: Cache::new()?,
        })
    }

    fn save_data_json(&self, data: &AwsRegionalProductServices) -> anyhow::Result<()> {
        let p = get_data_json_path();

        let data = serde_json::to_string_pretty(data)
            .with_context(|| format!("Failed to serialize data for {}", p.display()))?;
        std::fs::write(&p, data)
            .with_context(|| format!("Failed to write data to {}", p.display()))?;
        Ok(())
    }

    pub async fn retrieve(&self) -> anyhow::Result<AwsRegionalProductServices> {
        self.retrieve_with(RetrieveSource::Auto).await
    }

    pub async fn retrieve_with(
        &self,
        mode: RetrieveSource,
    ) -> anyhow::Result<AwsRegionalProductServices> {
        match mode {
            RetrieveSource::Auto => self.retrieve_fetch_or_cache().await,
            RetrieveSource::Fetch => self.retrieve_fetch().await,
        }
    }

    async fn retrieve_fetch(&self) -> anyhow::Result<AwsRegionalProductServices> {
        let data = aws_regional_product_services::fetch().await?;
        self.save_data_json(&data)?;
        Ok(data)
    }

    async fn retrive_cache(&self) -> anyhow::Result<AwsRegionalProductServices> {
        if let Some(data) = self.cache.get() {
            Ok(data.clone())
        } else {
            anyhow::bail!("No cache found");
        }
    }

    async fn retrieve_fetch_or_cache(&self) -> anyhow::Result<AwsRegionalProductServices> {
        let config = Config::create_or_load_default()?;

        if config.fetch_mode == FetchMode::Always {
            return self.retrieve_fetch().await;
        }

        if let Ok(data) = self.retrive_cache().await {
            Ok(data)
        } else {
            self.retrieve_fetch().await
        }
    }
}

/// Source to retrieve data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetrieveSource {
    /// Automatically choose the source to retrieve data.
    /// If cache is available, use it. Otherwise, fetch data.
    Auto,
    /// Always fetch data.
    Fetch,
}

impl Default for RetrieveSource {
    fn default() -> Self {
        Self::Auto
    }
}

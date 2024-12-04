use std::path::{Path, PathBuf};

use anyhow::Context;

pub fn init() -> anyhow::Result<()> {
    let dir = get_config_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create config directory: {}", dir.display()))?;
    }
    Ok(())
}

pub fn get_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("aws-regional-product-services")
}

pub fn get_data_json_path() -> PathBuf {
    let dir = get_config_dir();
    dir.join("data.json")
}

pub fn get_config_path() -> PathBuf {
    let dir = get_config_dir();
    dir.join("config.yaml")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub fetch_mode: FetchMode,
}

impl Config {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let config = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let config = serde_yaml::from_str(&config)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
        Ok(config)
    }

    pub fn load_default() -> anyhow::Result<Self> {
        Self::load(&get_config_path())
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let config = serde_yaml::to_string(self)?;
        std::fs::write(path, config)?;
        Ok(())
    }

    pub fn create_or_load_default() -> anyhow::Result<Self> {
        let path = get_config_path();
        if !path.exists() {
            let config = Self::default();
            config.save(&path)?;
        }
        Self::load(&path)
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    strum::VariantNames,
)]
#[strum(serialize_all = "kebab-case")]
pub enum FetchMode {
    Default,
    Always,
}

impl Default for FetchMode {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_config_dir() {
        let config_dir = get_config_dir();
        let config_dir = config_dir.to_str().unwrap();

        assert!(
            Regex::new(r"^C:\\Users\\[^\\]+\\.config\\aws-regional-product-services$")
                .unwrap()
                .is_match(config_dir)
        )
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_config_dir() {
        let config_dir = get_config_dir();
        let config_dir = config_dir.to_str().unwrap();

        assert!(
            Regex::new(r"^/home/[^/]+/.config/aws-regional-product-services$")
                .unwrap()
                .is_match(config_dir)
        )
    }

    #[test]
    fn test_fetch_mode_default() {
        let fetch_mode = FetchMode::default();
        assert_eq!(fetch_mode, FetchMode::Default);
    }
}

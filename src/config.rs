use std::path::PathBuf;

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
}

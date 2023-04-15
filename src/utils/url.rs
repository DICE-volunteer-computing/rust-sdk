use crate::config::config::{SdkConfig, Stage};

pub const INTEGRATION_ENDPOINT: &str = "http://api-lb-793478610.us-east-1.elb.amazonaws.com:80";
pub const DEV_ENDPOINT: &str = "http://localhost:8080";

pub fn path(config: SdkConfig, path: &str) -> String {
    match config.stage {
        Stage::Production => format!("{}/{}", INTEGRATION_ENDPOINT, path),
        Stage::Integration => format!("{}/{}", INTEGRATION_ENDPOINT, path),
        Stage::Dev => format!("{}/{}", DEV_ENDPOINT, path),
    }
}

pub fn create_path(config: SdkConfig, path_root: &str) -> String {
    path(config, path_root)
}

pub fn get_path(config: SdkConfig, path_root: &str, id: &str) -> String {
    path(config, format!("{}/{}", path_root, id).as_str())
}

pub fn list_path(config: SdkConfig, path_root: &str) -> String {
    path(config, format!("{}/list", path_root).as_str())
}

pub fn update_path(config: SdkConfig, path_root: &str, id: &str) -> String {
    path(config, format!("{}/{}/update", path_root, id).as_str())
}

pub fn download_path(config: SdkConfig, path_root: &str, id: &str) -> String {
    path(config, format!("{}/{}/download", path_root, id).as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_path() {
        assert_eq!(
            "http://api-lb-793478610.us-east-1.elb.amazonaws.com:80/hello/world",
            path(
                SdkConfig {
                    stage: Stage::Production
                },
                "hello/world"
            ),
        );

        assert_eq!(
            "http://api-lb-793478610.us-east-1.elb.amazonaws.com:80/hello/world",
            path(
                SdkConfig {
                    stage: Stage::Integration
                },
                "hello/world"
            ),
        );

        assert_eq!(
            "http://localhost:8080/hello/world",
            path(SdkConfig { stage: Stage::Dev }, "hello/world")
        );
    }

    #[test]
    pub fn test_create_path() {
        assert_eq!(
            "http://localhost:8080/rust",
            create_path(SdkConfig { stage: Stage::Dev }, "rust")
        );
    }

    #[test]
    pub fn test_get_path() {
        assert_eq!(
            "http://localhost:8080/rust/1234567",
            get_path(SdkConfig { stage: Stage::Dev }, "rust", "1234567")
        );
    }

    #[test]
    pub fn test_list_path() {
        assert_eq!(
            "http://localhost:8080/rust/list",
            list_path(SdkConfig { stage: Stage::Dev }, "rust")
        );
    }

    #[test]
    pub fn test_update_path() {
        assert_eq!(
            "http://localhost:8080/rust/1234567/update",
            update_path(SdkConfig { stage: Stage::Dev }, "rust", "1234567")
        );
    }

    #[test]
    pub fn test_download_path() {
        assert_eq!(
            "http://localhost:8080/rust/1234567/download",
            download_path(SdkConfig { stage: Stage::Dev }, "rust", "1234567")
        );
    }
}

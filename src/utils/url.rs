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

pub fn format_path_components(config: SdkConfig, components: Vec<&str>) -> String {
    path(config, &components.join("/"))
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
                    stage: Stage::Production,
                    auth: format!("")
                },
                "hello/world"
            ),
        );

        assert_eq!(
            "http://api-lb-793478610.us-east-1.elb.amazonaws.com:80/hello/world",
            path(
                SdkConfig {
                    stage: Stage::Integration,
                    auth: format!("")
                },
                "hello/world"
            ),
        );

        assert_eq!(
            "http://localhost:8080/hello/world",
            path(
                SdkConfig {
                    stage: Stage::Dev,
                    auth: format!("")
                },
                "hello/world"
            )
        );
    }

    #[test]
    pub fn test_format_path_components() {
        assert_eq!(
            "http://localhost:8080/rust/1234567",
            format_path_components(
                SdkConfig {
                    stage: Stage::Dev,
                    auth: format!(""),
                },
                vec!["rust", "1234567"],
            )
        );
    }
}

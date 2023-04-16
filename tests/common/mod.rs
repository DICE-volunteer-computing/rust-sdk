use rust_sdk::config::config::{SdkConfig, Stage};

pub const TEST_SDK_CONFIG: SdkConfig = SdkConfig {
    stage: Stage::Integration,
};

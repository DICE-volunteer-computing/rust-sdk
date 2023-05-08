use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Stage {
    Production,
    Integration,
    Dev,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SdkConfig {
    pub stage: Stage,
}

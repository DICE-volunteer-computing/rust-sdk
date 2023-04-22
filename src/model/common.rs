use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum PlatformArchitecture {
    X86_64,
    Arm64,
    Wasm,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PlatformExecutionType {
    Wasmer,
    Docker,
}

use std::collections::HashSet;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum PermissionsLevel {
    Read,
    Write,
    Admin,
}

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
    BareMetal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permissions {
    pub read: HashSet<ObjectId>,
    pub write: HashSet<ObjectId>,
    pub admin: HashSet<ObjectId>,
}

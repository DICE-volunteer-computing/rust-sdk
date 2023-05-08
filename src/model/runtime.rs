use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::common::{Permissions, PlatformArchitecture, PlatformExecutionType};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RuntimeStatus {
    CreatedPendingUpload,
    Active,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runtime {
    pub created_at: u64,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub last_updated_at: u64,
    pub permissions: Permissions,
    pub platform_architecture: PlatformArchitecture,
    pub platform_execution_type: PlatformExecutionType,
    pub project_id: ObjectId,
    pub status: RuntimeStatus,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRuntimeDTO {
    pub platform_architecture: PlatformArchitecture,
    pub platform_execution_type: PlatformExecutionType,
    pub project_id: ObjectId,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRuntimeResponse {
    pub id: ObjectId,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRuntimeDTO {
    pub status: Option<RuntimeStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRuntimeResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadRuntimeResponse {
    pub uri: String,
}

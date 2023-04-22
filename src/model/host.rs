use super::common::{PlatformArchitecture, PlatformExecutionType};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum HostStatus {
    Stale,
    Idle,
    Busy,
    Error,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Configuration {
    pub disk_bytes: u64,
    pub fp64_flops: Option<u64>,
    pub mem_bytes: u64,
    pub platform_architecture_types: Vec<PlatformArchitecture>,
    pub platform_execution_types: Vec<PlatformExecutionType>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Host {
    pub configuration: Configuration,
    pub created_at: u64,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub last_updated_at: u64,
    pub status: HostStatus,
    pub tags: HashMap<String, String>,
    pub user_id: ObjectId,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateHostDTO {
    pub configuration: Configuration,
    pub tags: HashMap<String, String>,
    pub user_id: ObjectId,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateHostResponse {
    pub id: ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHostDTO {
    pub status: Option<HostStatus>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateHostResponse {}

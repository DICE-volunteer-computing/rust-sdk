use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum Status {
    Stale,
    Idle,
    Busy,
    Error,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Configuration {
    pub mem_bytes: u64,
    pub disk_bytes: u64,
    pub cores: u64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Host {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub status: Status,
    pub last_updated_at: u64,
    pub configuration: Configuration,
    pub created_at: u64,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateHostDTO {
    pub user_id: String,
    pub configuration: Configuration,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHostDTO {
    pub status: Status,
}

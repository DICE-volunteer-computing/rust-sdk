use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    CreatedPendingUpload,
    Active,
    Inactive,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runtime {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub project_id: ObjectId,
    pub status: Status,
    pub created_at: u64,
    pub last_updated_at: u64,
    pub name: String,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRuntimeDTO {
    pub project_id: String,
    pub name: String,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRuntimeResponse {
    pub uri: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRuntimeDTO {
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadRuntimeResponse {
    pub uri: String,
}

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    Active,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub project_id: ObjectId,
    pub status: Status,
    pub input_artifacts: Vec<ObjectId>,
    pub runtime_id: ObjectId,
    pub tags: HashMap<String, String>,
    pub created_at: u64,
    pub last_updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobDTO {
    pub input_artifact_ids: Vec<String>,
    pub runtime_id: String,
    pub project_id: String,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobResponse {
    pub id: String,
}
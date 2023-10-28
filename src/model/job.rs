use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum JobStatus {
    Active,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub created_at: u64,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub input_artifacts: Vec<ObjectId>,
    pub last_updated_at: u64,
    pub project_id: ObjectId,
    pub runtime_id: ObjectId,
    pub status: JobStatus,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobDTO {
    pub input_artifact_ids: Vec<ObjectId>,
    pub runtime_id: ObjectId,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobResponse {
    pub id: ObjectId,
}

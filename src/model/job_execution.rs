use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    PendingExecution,
    Execution,
    PendingArtifactUpload,
    Completed,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobExecution {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub job_id: ObjectId,
    pub host_id: ObjectId,
    pub tags: HashMap<String, String>,
    pub status: Status,
    pub created_at: u64,
    pub last_updated_at: u64,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobExecutionDTO {
    pub job_id: String,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobExecutionResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateJobExecutionDTO {
    pub status: Status,
}

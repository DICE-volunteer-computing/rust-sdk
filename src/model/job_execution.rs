use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum JobExecutionStatus {
    PendingExecution,
    Execution,
    PendingArtifactUpload,
    Completed,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobExecution {
    pub created_at: u64,
    pub end_time: u64,
    pub host_id: ObjectId,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub job_id: ObjectId,
    pub last_updated_at: u64,
    pub output_artifacts: Vec<ObjectId>,
    pub start_time: u64,
    pub status: JobExecutionStatus,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobExecutionDTO {
    pub job_id: ObjectId,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobExecutionResponse {
    pub id: ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateJobExecutionDTO {
    pub output_artifacts: Option<Vec<ObjectId>>,
    pub status: Option<JobExecutionStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateJobExecutionResponse {}

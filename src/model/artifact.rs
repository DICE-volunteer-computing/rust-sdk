use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ArtifactStatus {
    CreatedPendingUpload,
    Active,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
    pub created_at: u64,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub last_updated_at: u64,
    pub project_id: ObjectId,
    pub status: ArtifactStatus,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateArtifactDTO {
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateArtifactResponse {
    pub id: ObjectId,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateArtifactDTO {
    pub status: Option<ArtifactStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateArtifactResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadArtifactResponse {
    pub id: ObjectId,
    pub uri: String,
}

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::entity::EntityType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    CreatedPendingUpload,
    Active,
    Inactive,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ArtifactType {
    Input,
    Output,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub status: Status,
    pub artifact_type: ArtifactType,
    pub entity_id: ObjectId,
    pub entity_type: EntityType,
    pub created_at: u64,
    pub last_updated_at: u64,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateArtifactDTO {
    pub entity_id: String,
    pub entity_type: EntityType,
    pub artifact_type: ArtifactType,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateArtifactResponse {
    pub uri: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateArtifactDTO {
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadArtifactResponse {
    pub uri: String,
}

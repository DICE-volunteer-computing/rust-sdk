use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::common::Permissions;

#[derive(Serialize, Debug, Deserialize)]
pub struct Project {
    pub created_at: u64,
    pub description: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub last_updated_at: u64,
    pub name: String,
    pub permissions: Permissions,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateProjectDTO {
    pub description: String,
    pub name: String,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateProjectResponse {
    pub id: ObjectId,
}

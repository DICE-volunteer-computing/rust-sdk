use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Project {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub description: String,
    pub user_id: ObjectId,
    pub tags: HashMap<String, String>,
    pub created_at: u64,
    pub last_updated_at: u64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateProjectDTO {
    pub description: String,
    pub tags: HashMap<String, String>,
}

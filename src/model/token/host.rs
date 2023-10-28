use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HostToken {
    pub created_at: u64,
    pub hash: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub host_id: ObjectId,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateHostTokenDTO {
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateHostTokenResponse {
    pub token: String,
}

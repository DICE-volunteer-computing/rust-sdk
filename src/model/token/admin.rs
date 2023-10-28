use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminToken {
    pub created_at: u64,
    pub hash: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAdminTokenDTO {
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAdminTokenResponse {
    pub token: String,
}

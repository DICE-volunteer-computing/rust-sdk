use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HostRegistrationToken {
    pub created_at: u64,
    pub hash: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub user_id: ObjectId,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateHostRegistrationTokenDTO {
    pub name: String,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateHostRegistrationTokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteHostRegistrationTokenResponse {}

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::common::Permissions;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TokenApplicationType {
    Api,
    HostRegistration,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TokenStatus {
    Active,
    Inactive,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub created_at: u64,
    pub hash: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub is_admin: bool,
    pub last_updated_at: u64,
    pub name: String,
    pub permissions: Permissions,
    pub status: TokenStatus,
    pub tags: HashMap<String, String>,
    pub token_application_type: TokenApplicationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateApiTokenDTO {
    pub name: String,
    pub tags: HashMap<String, String>,
    pub user_id: ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateHostRegistrationTokenDTO {
    pub name: String,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTokenDTO {
    pub status: Option<TokenStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTokenResponse {}

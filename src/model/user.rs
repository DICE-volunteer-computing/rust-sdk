use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum ResearcherStatus {
    Active,
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum ComputeProviderStatus {
    Active,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct User {
    pub compute_provider_status: ComputeProviderStatus,
    pub created_at: u64,
    pub email: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub last_updated_at: u64,
    pub researcher_status: ResearcherStatus,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateUserDTO {
    pub email: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateUserResponse {
    pub id: ObjectId,
}

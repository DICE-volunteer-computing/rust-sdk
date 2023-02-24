use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum DocumentType {
    UserCreditConsumerAttestation,
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum Status {
    Uploaded,
    NotUploaded,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Document {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub status: Status,
    pub created_at: u64,
    pub entity_id: ObjectId,
    pub document_type: DocumentType,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateDocumentDTO {
    pub document_type: DocumentType,
    pub entity_id: String,
    pub tags: HashMap<String, String>,
}

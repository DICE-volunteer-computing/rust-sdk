use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::entity::EntityType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub entity_id: ObjectId,
    pub entity_type: EntityType,
    pub timestamp: u64,
    pub original: Value,
    pub updated: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEventDTO {
    pub entity_id: String,
    pub entity_type: EntityType,
    pub original: Value,
    pub updated: Value,
}

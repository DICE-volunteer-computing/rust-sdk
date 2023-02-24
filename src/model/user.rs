use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum AccountType {
    Individual,
    Business,
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum CreditConsumerStatus {
    PendingActivation,
    Active,
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum CreditProducerStatus {
    Active,
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub enum SponsorshipTier {
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub account_type: AccountType,
    pub credit_consumer_status: CreditConsumerStatus,
    pub credit_producer_status: CreditProducerStatus,
    pub sponsorship_tier: SponsorshipTier,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateUserDTO {
    pub email: String,
    pub account_type: AccountType,
}

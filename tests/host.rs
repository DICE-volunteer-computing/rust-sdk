use std::{collections::HashMap, str::FromStr};

mod common;

use crate::common::TEST_SDK_CONFIG;
use mongodb::bson::{doc, oid::ObjectId};
use rust_sdk::{
    api::host::{create, get, list, update},
    model::host::{Configuration, CreateHostDTO, HostStatus, UpdateHostDTO},
    utils::time::seconds,
};

#[tokio::test]
async fn test_crud_host() {
    let start_time = seconds() - 1;

    // Create a new host
    let data = CreateHostDTO {
        user_id: ObjectId::from_str("63f6bc062c4b38df844c9593")
            .expect("could not convert String to ObjectId"),
        configuration: Configuration {
            mem_bytes: 65432,
            disk_bytes: 987654,
        },
        tags: HashMap::new(),
    };
    let host_id = create(TEST_SDK_CONFIG, data).await.id;

    // Get the host, validate its properties
    let host = get(TEST_SDK_CONFIG, host_id).await;

    assert_eq!(host.configuration.mem_bytes, 65432);
    assert_eq!(host.configuration.disk_bytes, 987654);
    assert!(host.created_at > start_time);
    assert!(host.last_updated_at > start_time);
    assert_eq!(host.status, HostStatus::Stale);
    assert_eq!(host.tags, HashMap::new());

    // Update the host, validate its properties
    update(
        TEST_SDK_CONFIG,
        host_id,
        UpdateHostDTO {
            status: Some(HostStatus::Idle),
        },
    )
    .await;
    let updated_host = get(TEST_SDK_CONFIG, host_id).await;

    assert_eq!(updated_host.configuration.mem_bytes, 65432);
    assert_eq!(updated_host.configuration.disk_bytes, 987654);
    assert!(updated_host.created_at > start_time);
    assert!(updated_host.last_updated_at >= host.last_updated_at);
    assert_eq!(updated_host.status, HostStatus::Idle);
    assert_eq!(updated_host.tags, HashMap::new());

    // List hosts
    let hosts = list(
        TEST_SDK_CONFIG,
        doc! {
            "_id": host_id
        },
    )
    .await;

    assert_eq!(hosts.len(), 1);
}

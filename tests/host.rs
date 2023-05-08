mod common;

use std::collections::HashMap;

use crate::common::TEST_SDK_CONFIG;
use mongodb::bson::doc;
use rust_sdk::{
    api::host::{create, get, list, update},
    model::{
        common::{PlatformArchitecture, PlatformExecutionType},
        host::{Configuration, CreateHostDTO, HostStatus, UpdateHostDTO},
    },
    utils::time::seconds,
};

#[tokio::test]
async fn test_crud_host() {
    let start_time = seconds() - 1;

    // Create a new host
    let data = CreateHostDTO {
        configuration: Configuration {
            disk_bytes: 987654,
            mem_bytes: 65432,
            platform_architecture_types: vec![
                PlatformArchitecture::Arm64,
                PlatformArchitecture::Wasm,
            ],
            platform_execution_types: vec![
                PlatformExecutionType::Docker,
                PlatformExecutionType::Wasmer,
            ],
        },
        tags: HashMap::new(),
    };
    let host_id = create(TEST_SDK_CONFIG, data).await.id;

    // Get the host, validate its properties
    let host = get(TEST_SDK_CONFIG, host_id).await;

    assert_eq!(host.configuration.disk_bytes, 987654);
    assert_eq!(host.configuration.mem_bytes, 65432);
    assert_eq!(
        host.configuration.platform_architecture_types,
        vec![PlatformArchitecture::Arm64, PlatformArchitecture::Wasm]
    );
    assert_eq!(
        host.configuration.platform_execution_types,
        vec![PlatformExecutionType::Docker, PlatformExecutionType::Wasmer]
    );
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

    assert_eq!(updated_host.status, HostStatus::Idle);

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

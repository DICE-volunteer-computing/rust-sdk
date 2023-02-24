use std::collections::HashMap;

use rust_sdk::{
    api::host::{create, get, update},
    model::host::{Configuration, CreateHostDTO, Status, UpdateHostDTO},
    utils::time::seconds,
};

#[tokio::test]
async fn test_crud_host() {
    let start_time = seconds() - 1;

    // Create a new host
    let data = CreateHostDTO {
        user_id: format!("63f6bc062c4b38df844c9593"),
        configuration: Configuration {
            mem_bytes: 65432,
            disk_bytes: 987654,
            cores: 4,
        },
        tags: HashMap::new(),
    };
    let host_id = create(data).await;

    // Get the host, validate its properties
    let host = get(host_id.clone()).await;

    assert_eq!(host.configuration.mem_bytes, 65432);
    assert_eq!(host.configuration.disk_bytes, 987654);
    assert_eq!(host.configuration.cores, 4);
    assert!(host.created_at > start_time);
    assert!(host.status.last_updated_at > start_time);
    assert_eq!(host.status.state, Status::Stale);
    assert_eq!(host.tags, HashMap::new());

    // Update the host, validate its properties
    update(
        host_id.clone(),
        UpdateHostDTO {
            state: Status::Idle,
        },
    )
    .await;
    let updated_host = get(host_id.clone()).await;

    assert_eq!(updated_host.configuration.mem_bytes, 65432);
    assert_eq!(updated_host.configuration.disk_bytes, 987654);
    assert_eq!(updated_host.configuration.cores, 4);
    assert!(updated_host.created_at > start_time);
    assert!(updated_host.status.last_updated_at >= host.status.last_updated_at);
    assert_eq!(updated_host.status.state, Status::Idle);
    assert_eq!(updated_host.tags, HashMap::new());
}

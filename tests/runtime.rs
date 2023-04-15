use std::collections::HashMap;

use mongodb::bson::doc;
use rust_sdk::{
    api::{project, runtime},
    config::config::{SdkConfig, Stage},
    model::{
        project::CreateProjectDTO,
        runtime::{CreateRuntimeDTO, Status, UpdateRuntimeDTO},
    },
    utils::time::seconds,
};

#[tokio::test]
async fn test_crud_runtime() {
    let start_time = seconds() - 1;

    // --- DEPENDENCIES ---
    let project_id = project::create(
        SdkConfig {
            stage: Stage::Integration,
        },
        CreateProjectDTO {
            description: format!("test 1"),
            tags: HashMap::new(),
        },
    )
    .await;

    // --- CREATE ---
    let data = CreateRuntimeDTO {
        project_id: project_id.clone(),
        name: format!("test"),
        tags: HashMap::new(),
    };
    let create_runtime_response = runtime::create(
        SdkConfig {
            stage: Stage::Integration,
        },
        data,
    )
    .await;

    // --- GET ---
    let runtime = runtime::get(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_runtime_response.id.as_str(),
    )
    .await;

    assert_eq!(runtime.project_id.to_string(), project_id);
    assert!(runtime.created_at > start_time);
    assert!(runtime.last_updated_at > start_time);
    assert_eq!(runtime.status, Status::CreatedPendingUpload);
    assert_eq!(runtime.tags, HashMap::new());

    // --- UPDATE ---
    let update_data = UpdateRuntimeDTO {
        status: Status::Active,
    };

    runtime::update(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_runtime_response.id.as_str(),
        update_data,
    )
    .await;

    let updated_runtime = runtime::get(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_runtime_response.id.as_str(),
    )
    .await;
    assert_eq!(updated_runtime.status, Status::Active);

    // --- LIST ---
    let runtimes = runtime::list(
        SdkConfig {
            stage: Stage::Integration,
        },
        doc! { "_id": {
            "$oid": create_runtime_response.id.clone()
        }},
    )
    .await;

    assert_eq!(runtimes.len(), 1);

    // --- DOWNLOAD ---
    runtime::download(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_runtime_response.id.as_str(),
    )
    .await;
}

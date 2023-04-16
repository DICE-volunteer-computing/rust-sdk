use std::collections::HashMap;

mod common;

use mongodb::bson::doc;
use rust_sdk::{
    api::{project, runtime},
    config::config::{SdkConfig, Stage},
    model::{
        project::CreateProjectDTO,
        runtime::{CreateRuntimeDTO, RuntimeExecutionType, RuntimeStatus, UpdateRuntimeDTO},
    },
    utils::time::seconds,
};

use crate::common::TEST_SDK_CONFIG;

#[tokio::test]
async fn test_crud_runtime() {
    let start_time = seconds() - 1;

    // --- DEPENDENCIES ---
    let project_id = project::create(
        TEST_SDK_CONFIG,
        CreateProjectDTO {
            description: format!("test 1"),
            name: format!("test name 1"),
            tags: HashMap::new(),
        },
    )
    .await
    .id;

    // --- CREATE ---
    let data = CreateRuntimeDTO {
        execution_type: RuntimeExecutionType::Wasmer,
        project_id: project_id,
        tags: HashMap::new(),
    };
    let create_runtime_response = runtime::create(TEST_SDK_CONFIG, data).await;

    // --- GET ---
    let runtime = runtime::get(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_runtime_response.id,
    )
    .await;

    assert_eq!(runtime.project_id, project_id);
    assert!(runtime.created_at > start_time);
    assert!(runtime.last_updated_at > start_time);
    assert_eq!(runtime.execution_type, RuntimeExecutionType::Wasmer);
    assert_eq!(runtime.status, RuntimeStatus::CreatedPendingUpload);
    assert_eq!(runtime.tags, HashMap::new());

    // --- UPDATE ---
    let update_data = UpdateRuntimeDTO {
        status: Some(RuntimeStatus::Active),
    };

    runtime::update(TEST_SDK_CONFIG, create_runtime_response.id, update_data).await;

    let updated_runtime = runtime::get(TEST_SDK_CONFIG, create_runtime_response.id).await;
    assert_eq!(updated_runtime.status, RuntimeStatus::Active);

    // --- LIST ---
    let runtimes = runtime::list(
        TEST_SDK_CONFIG,
        doc! {
            "_id": create_runtime_response.id
        },
    )
    .await;

    assert_eq!(runtimes.len(), 1);

    // --- DOWNLOAD ---
    runtime::download(TEST_SDK_CONFIG, create_runtime_response.id).await;
}

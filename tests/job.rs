use std::collections::HashMap;

mod common;

use mongodb::bson::doc;
use rust_sdk::{
    api::{artifact, job, project, runtime},
    model::{
        artifact::{ArtifactStatus, CreateArtifactDTO, UpdateArtifactDTO},
        common::{PlatformArchitecture, PlatformExecutionType},
        job::{CreateJobDTO, JobStatus},
        project::CreateProjectDTO,
        runtime::{CreateRuntimeDTO, RuntimeStatus, UpdateRuntimeDTO},
    },
    utils::time::seconds,
};

use crate::common::TEST_SDK_CONFIG;

#[tokio::test]
async fn test_crud_job() {
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

    let create_artifact_response = artifact::create(
        TEST_SDK_CONFIG,
        CreateArtifactDTO {
            tags: HashMap::new(),
        },
    )
    .await;
    artifact::update(
        TEST_SDK_CONFIG,
        create_artifact_response.id,
        UpdateArtifactDTO {
            status: Some(ArtifactStatus::Active),
        },
    )
    .await;

    let create_runtime_response = runtime::create(
        TEST_SDK_CONFIG,
        CreateRuntimeDTO {
            platform_architecture: PlatformArchitecture::Wasm,
            platform_execution_type: PlatformExecutionType::Wasmer,
            project_id: project_id,
            tags: HashMap::new(),
        },
    )
    .await;
    runtime::update(
        TEST_SDK_CONFIG,
        create_runtime_response.id,
        UpdateRuntimeDTO {
            status: Some(RuntimeStatus::Active),
        },
    )
    .await;

    // --- CREATE ---
    let create_job_response = job::create(
        TEST_SDK_CONFIG,
        CreateJobDTO {
            input_artifact_ids: vec![create_artifact_response.id],
            runtime_id: create_runtime_response.id,
            project_id: project_id,
            tags: HashMap::new(),
        },
    )
    .await;

    // --- GET ---
    let job = job::get(TEST_SDK_CONFIG, create_job_response.id).await;

    assert_eq!(job.project_id, project_id);
    assert!(job.created_at > start_time);
    assert!(job.last_updated_at > start_time);
    assert_eq!(job.status, JobStatus::Active);
    assert_eq!(job.tags, HashMap::new());
    assert_eq!(job.runtime_id, create_runtime_response.id);
    assert_eq!(job.input_artifacts.len(), 1);
    assert_eq!(job.input_artifacts[0], create_artifact_response.id);

    // --- LIST ---
    let jobs = job::list(
        TEST_SDK_CONFIG,
        doc! {
            "_id": create_job_response.id
        },
    )
    .await;

    assert_eq!(jobs.len(), 1);
}

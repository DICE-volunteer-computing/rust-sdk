use std::collections::HashMap;

mod common;
use crate::common::TEST_SDK_CONFIG;

use mongodb::bson::doc;
use rust_sdk::{
    api::{artifact, job, job_execution, project, runtime},
    model::{
        artifact::{ArtifactStatus, CreateArtifactDTO, UpdateArtifactDTO},
        job::CreateJobDTO,
        job_execution::{CreateJobExecutionDTO, JobExecutionStatus, UpdateJobExecutionDTO},
        project::CreateProjectDTO,
        runtime::{CreateRuntimeDTO, RuntimeExecutionType, RuntimeStatus, UpdateRuntimeDTO},
    },
    utils::time::seconds,
};

#[tokio::test]
async fn test_crud_job_execution() {
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
            execution_type: RuntimeExecutionType::Wasmer,
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

    // --- CREATE ---
    let create_job_execution_response = job_execution::create(
        TEST_SDK_CONFIG,
        CreateJobExecutionDTO {
            job_id: create_job_response.id,
            tags: HashMap::new(),
        },
    )
    .await;

    // --- GET ---
    let job_execution = job_execution::get(TEST_SDK_CONFIG, create_job_execution_response.id).await;

    assert!(job_execution.created_at > start_time);
    assert!(job_execution.last_updated_at > start_time);
    assert_eq!(job_execution.status, JobExecutionStatus::PendingExecution);
    assert_eq!(job_execution.tags, HashMap::new());
    assert_eq!(job_execution.job_id, create_job_response.id);
    assert_eq!(job_execution.start_time, 0);
    assert_eq!(job_execution.end_time, 0);

    // --- UPDATE ---
    job_execution::update(
        TEST_SDK_CONFIG,
        job_execution.id,
        UpdateJobExecutionDTO {
            output_artifacts: None,
            status: Some(JobExecutionStatus::Execution),
        },
    )
    .await;
    let updated_job_execution = job_execution::get(TEST_SDK_CONFIG, job_execution.id).await;

    assert_eq!(updated_job_execution.status, JobExecutionStatus::Execution);

    // --- LIST ---
    let job_executions = job_execution::list(
        TEST_SDK_CONFIG,
        doc! {
            "_id": create_job_execution_response.id
        },
    )
    .await;

    assert_eq!(job_executions.len(), 1);
}

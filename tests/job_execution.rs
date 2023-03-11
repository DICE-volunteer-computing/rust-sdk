use std::collections::HashMap;

use mongodb::bson::doc;
use rust_sdk::{
    api::{artifact, job, job_execution, project, runtime},
    model::{
        artifact::{ArtifactType, CreateArtifactDTO, Status as ArtifactStatus, UpdateArtifactDTO},
        entity::EntityType,
        job::CreateJobDTO,
        job_execution::{CreateJobExecutionDTO, Status, UpdateJobExecutionDTO},
        project::CreateProjectDTO,
        runtime::{CreateRuntimeDTO, Status as RuntimeStatus, UpdateRuntimeDTO},
    },
    utils::time::seconds,
};

#[tokio::test]
async fn test_crud_job_execution() {
    let start_time = seconds() - 1;

    // --- DEPENDENCIES ---
    let project_id = project::create(CreateProjectDTO {
        description: format!("test 1"),
        tags: HashMap::new(),
    })
    .await;

    let create_artifact_response = artifact::create(CreateArtifactDTO {
        entity_id: project_id.clone(),
        entity_type: EntityType::Project,
        artifact_type: ArtifactType::Input,
        tags: HashMap::new(),
    })
    .await;
    artifact::update(
        create_artifact_response.id.clone(),
        UpdateArtifactDTO {
            status: ArtifactStatus::Active,
        },
    )
    .await;

    let create_runtime_response = runtime::create(CreateRuntimeDTO {
        project_id: project_id.clone(),
        name: format!("test"),
        tags: HashMap::new(),
    })
    .await;
    runtime::update(
        create_runtime_response.id.clone(),
        UpdateRuntimeDTO {
            status: RuntimeStatus::Active,
        },
    )
    .await;

    let create_job_response = job::create(CreateJobDTO {
        input_artifact_ids: vec![create_artifact_response.id.clone()],
        runtime_id: create_runtime_response.id.clone(),
        project_id: project_id.clone(),
        tags: HashMap::new(),
    })
    .await;

    // --- CREATE ---
    let create_job_execution_response = job_execution::create(CreateJobExecutionDTO {
        job_id: create_job_response.id.clone(),
        tags: HashMap::new(),
    })
    .await;

    // --- GET ---
    let job_execution = job_execution::get(create_job_execution_response.id.clone()).await;

    assert!(job_execution.created_at > start_time);
    assert!(job_execution.last_updated_at > start_time);
    assert_eq!(job_execution.status, Status::PendingExecution);
    assert_eq!(job_execution.tags, HashMap::new());
    assert_eq!(job_execution.job_id.to_string(), create_job_response.id);
    assert_eq!(job_execution.start_time, 0);
    assert_eq!(job_execution.end_time, 0);

    // --- UPDATE ---
    job_execution::update(
        job_execution.id.clone().to_string(),
        UpdateJobExecutionDTO {
            status: Status::Execution,
        },
    )
    .await;
    let updated_job_execution = job_execution::get(job_execution.id.clone().to_string()).await;

    assert_eq!(updated_job_execution.status, Status::Execution);

    // --- LIST ---
    let job_executions = job_execution::list(doc! { "_id": {
        "$oid": create_job_execution_response.id.clone()
    }})
    .await;

    assert_eq!(job_executions.len(), 1);
}

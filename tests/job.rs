use std::collections::HashMap;

use mongodb::bson::doc;
use rust_sdk::{
    api::{artifact, job, project, runtime},
    model::{
        artifact::{ArtifactType, CreateArtifactDTO, Status as ArtifactStatus, UpdateArtifactDTO},
        entity::EntityType,
        job::{CreateJobDTO, Status},
        project::CreateProjectDTO,
        runtime::{CreateRuntimeDTO, Status as RuntimeStatus, UpdateRuntimeDTO},
    },
    utils::time::seconds,
};

#[tokio::test]
async fn test_crud_job() {
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

    // --- CREATE ---
    let create_job_response = job::create(CreateJobDTO {
        input_artifact_ids: vec![create_artifact_response.id.clone()],
        runtime_id: create_runtime_response.id.clone(),
        project_id: project_id.clone(),
        tags: HashMap::new(),
    })
    .await;

    // --- GET ---
    let job = job::get(create_job_response.id.clone()).await;

    assert_eq!(job.project_id.to_string(), project_id);
    assert!(job.created_at > start_time);
    assert!(job.last_updated_at > start_time);
    assert_eq!(job.status, Status::Active);
    assert_eq!(job.tags, HashMap::new());
    assert_eq!(job.runtime_id.to_string(), create_runtime_response.id);
    assert_eq!(job.input_artifacts.len(), 1);
    assert_eq!(
        job.input_artifacts[0].to_string(),
        create_artifact_response.id
    );

    // --- LIST ---
    let jobs = job::list(doc! { "_id": {
        "$oid": create_job_response.id.clone()
    }})
    .await;

    assert_eq!(jobs.len(), 1);
}

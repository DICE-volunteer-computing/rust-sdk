use std::collections::HashMap;

use mongodb::bson::doc;
use rust_sdk::{
    api::{artifact, project},
    config::config::{SdkConfig, Stage},
    model::{
        artifact::{ArtifactType, CreateArtifactDTO, Status, UpdateArtifactDTO},
        entity::EntityType,
        project::CreateProjectDTO,
    },
    utils::time::seconds,
};

#[tokio::test]
async fn test_crud_artifact() {
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
    let data = CreateArtifactDTO {
        entity_id: project_id.clone(),
        entity_type: EntityType::Project,
        artifact_type: ArtifactType::Input,
        tags: HashMap::new(),
    };
    let create_artifact_response = artifact::create(
        SdkConfig {
            stage: Stage::Integration,
        },
        data,
    )
    .await;

    // --- GET ---
    let artifact = artifact::get(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_artifact_response.id.as_str(),
    )
    .await;

    assert_eq!(artifact.entity_id.to_string(), project_id);
    assert!(artifact.created_at > start_time);
    assert!(artifact.last_updated_at > start_time);
    assert_eq!(artifact.status, Status::CreatedPendingUpload);
    assert_eq!(artifact.tags, HashMap::new());
    assert_eq!(artifact.artifact_type, ArtifactType::Input);

    // --- UPDATE ---
    let update_data = UpdateArtifactDTO {
        status: Status::Active,
    };

    artifact::update(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_artifact_response.id.as_str(),
        update_data,
    )
    .await;

    let updated_artifact = artifact::get(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_artifact_response.id.as_str(),
    )
    .await;
    assert_eq!(updated_artifact.status, Status::Active);

    // --- LIST ---
    let artifacts = artifact::list(
        SdkConfig {
            stage: Stage::Integration,
        },
        doc! { "_id": {
            "$oid": create_artifact_response.id.clone()
        }},
    )
    .await;

    assert_eq!(artifacts.len(), 1);

    // --- DOWNLOAD ---
    artifact::download(
        SdkConfig {
            stage: Stage::Integration,
        },
        create_artifact_response.id.as_str(),
    )
    .await;
}

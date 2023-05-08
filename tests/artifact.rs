use std::collections::HashMap;

mod common;

use mongodb::bson::doc;
use rust_sdk::{
    api::{artifact, project},
    model::{
        artifact::{ArtifactStatus, CreateArtifactDTO, UpdateArtifactDTO},
        project::CreateProjectDTO,
    },
    utils::time::seconds,
};

use crate::common::TEST_SDK_CONFIG;

#[tokio::test]
async fn test_crud_artifact() {
    let start_time = seconds() - 1;

    // --- DEPENDENCY (PROJECT) ---
    let project_id = project::create(
        TEST_SDK_CONFIG,
        CreateProjectDTO {
            description: format!("test description"),
            name: format!("test name 1"),
            tags: HashMap::new(),
        },
    )
    .await
    .id;

    // --- CREATE ---
    let data = CreateArtifactDTO {
        project_id: project_id,
        tags: HashMap::new(),
    };
    let create_artifact_response = artifact::create(TEST_SDK_CONFIG, data).await;

    // --- GET ---
    let artifact = artifact::get(TEST_SDK_CONFIG, create_artifact_response.id).await;

    assert!(artifact.created_at > start_time);
    assert!(artifact.last_updated_at > start_time);
    assert_eq!(artifact.status, ArtifactStatus::CreatedPendingUpload);
    assert_eq!(artifact.tags, HashMap::new());

    // --- UPDATE ---
    let update_data = UpdateArtifactDTO {
        status: Some(ArtifactStatus::Active),
    };

    artifact::update(TEST_SDK_CONFIG, create_artifact_response.id, update_data).await;

    let updated_artifact = artifact::get(TEST_SDK_CONFIG, create_artifact_response.id).await;
    assert_eq!(updated_artifact.status, ArtifactStatus::Active);

    // --- LIST ---
    let artifacts = artifact::list(
        TEST_SDK_CONFIG,
        doc! {
            "_id": create_artifact_response.id
        },
    )
    .await;

    assert_eq!(artifacts.len(), 1);

    // --- DOWNLOAD ---
    artifact::download(TEST_SDK_CONFIG, create_artifact_response.id).await;
}

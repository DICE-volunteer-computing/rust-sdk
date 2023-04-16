use std::collections::HashMap;

mod common;

use crate::common::TEST_SDK_CONFIG;

use mongodb::bson::doc;
use rust_sdk::{api::project, model::project::CreateProjectDTO, utils::time::seconds};

#[tokio::test]
async fn test_crud_project() {
    let start_time = seconds() - 1;

    // --- CREATE ---
    let input = CreateProjectDTO {
        description: format!("test description"),
        name: format!("test name 1"),
        tags: HashMap::new(),
    };

    let project_id = project::create(TEST_SDK_CONFIG, input).await.id;

    // --- GET ---
    let original_project = project::get(TEST_SDK_CONFIG, project_id).await;

    assert_eq!(original_project.description, format!("test description"));
    assert_eq!(original_project.name, format!("test name 1"));
    assert_eq!(original_project.tags, HashMap::new());
    assert!(original_project.created_at > start_time);
    assert!(original_project.last_updated_at > start_time);

    // --- LIST ---
    let projects = project::list(
        TEST_SDK_CONFIG,
        doc! {
            "_id": project_id
        },
    )
    .await;

    assert_eq!(projects.len(), 1);
}

use std::collections::HashMap;

use mongodb::bson::doc;
use rust_sdk::{api::project, model::project::CreateProjectDTO, utils::time::seconds};

#[tokio::test]
async fn test_crud_project() {
    let start_time = seconds() - 1;

    // --- CREATE ---
    let input = CreateProjectDTO {
        description: format!("test description"),
        tags: HashMap::new(),
    };

    let project_id = project::create(input).await;

    // --- GET ---
    let original_project = project::get(project_id.clone()).await;

    assert_eq!(original_project.description, format!("test description"));
    assert_eq!(original_project.tags, HashMap::new());
    assert!(original_project.created_at > start_time);
    assert!(original_project.last_updated_at > start_time);

    // --- LIST ---
    let projects = project::list(doc! { "_id": {
        "$oid": project_id
    }})
    .await;

    assert_eq!(projects.len(), 1);
}

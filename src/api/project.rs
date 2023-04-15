use mongodb::bson::Document;
use serde_json::Value;

use crate::{
    config::config::SdkConfig,
    model::project::{CreateProjectDTO, Project},
    utils::url::{create_path, get_path, list_path},
};

pub const PROJECT_PATH_ROOT: &str = "project";

pub async fn create(config: SdkConfig, input: CreateProjectDTO) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post(create_path(config, PROJECT_PATH_ROOT))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => serde_json::from_str::<Value>(&response.text().await.unwrap()).unwrap()
            ["insertedId"]["$oid"]
            .to_string()
            .replace("\"", ""),
        Err(_) => panic!("could not create project"),
    }
}

pub async fn get(config: SdkConfig, id: &str) -> Project {
    let client = reqwest::Client::new();
    let res = client
        .get(get_path(config, PROJECT_PATH_ROOT, id))
        .send()
        .await;

    match res {
        Ok(response) => serde_json::from_str(&response.text().await.unwrap()).unwrap(),
        Err(_) => panic!("could not get project"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Project> {
    let client = reqwest::Client::new();
    let res = client
        .post(list_path(config, PROJECT_PATH_ROOT))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => serde_json::from_str(&response.text().await.unwrap()).unwrap(),
        Err(_) => panic!("could not list projects"),
    }
}

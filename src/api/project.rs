use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::config::SdkConfig,
    model::project::{CreateProjectDTO, CreateProjectResponse, Project},
    utils::{
        auth::{add_auth, get_api_token},
        url::{create_path, get_path, list_path},
    },
};

pub const PROJECT_PATH_ROOT: &str = "project";

pub async fn create(config: SdkConfig, input: CreateProjectDTO) -> CreateProjectResponse {
    let client = reqwest::Client::new();
    let res = add_auth(
        client
            .post(create_path(config, PROJECT_PATH_ROOT))
            .json(&input),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse CreateProjectResponse"),
        Err(_) => panic!("could not create project"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> Project {
    let client = reqwest::Client::new();
    let res = add_auth(
        client.get(get_path(config, PROJECT_PATH_ROOT, id.to_string().as_str())),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse Project"),
        Err(_) => panic!("could not get project"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Project> {
    let client = reqwest::Client::new();
    let res = add_auth(
        client
            .post(list_path(config, PROJECT_PATH_ROOT))
            .json(&input),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse Vec<Project>"),
        Err(_) => panic!("could not list projects"),
    }
}

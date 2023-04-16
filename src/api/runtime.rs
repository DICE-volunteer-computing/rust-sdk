use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::config::SdkConfig,
    model::runtime::{
        CreateRuntimeDTO, CreateRuntimeResponse, DownloadRuntimeResponse, Runtime, UpdateRuntimeDTO,
    },
    utils::url::{create_path, download_path, get_path, list_path, update_path},
};

pub const RUNTIME_PATH_ROOT: &str = "runtime";

pub async fn create(config: SdkConfig, input: CreateRuntimeDTO) -> CreateRuntimeResponse {
    let client = reqwest::Client::new();
    let res = client
        .post(create_path(config, RUNTIME_PATH_ROOT))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse CreateRuntimeResponse"),
        Err(_) => panic!("could not create runtime"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> Runtime {
    let client = reqwest::Client::new();
    let res = client
        .get(get_path(config, RUNTIME_PATH_ROOT, id.to_string().as_str()))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse Runtime"),
        Err(_) => panic!("could not get runtime"),
    }
}

pub async fn update(config: SdkConfig, id: ObjectId, input: UpdateRuntimeDTO) {
    let client = reqwest::Client::new();
    let res = client
        .post(update_path(
            config,
            RUNTIME_PATH_ROOT,
            id.to_string().as_str(),
        ))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update runtime"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Runtime> {
    let client = reqwest::Client::new();
    let res = client
        .post(list_path(config, RUNTIME_PATH_ROOT))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse Vec<Runtime>"),
        Err(_) => panic!("could not list runtimes"),
    }
}

pub async fn download(config: SdkConfig, id: ObjectId) -> DownloadRuntimeResponse {
    let client = reqwest::Client::new();
    let res = client
        .get(download_path(
            config,
            RUNTIME_PATH_ROOT,
            id.to_string().as_str(),
        ))
        .send()
        .await;

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse DownloadRuntimeResponse"),
        Err(_) => panic!("could not get runtime"),
    }
}

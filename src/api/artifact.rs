use mongodb::bson::Document;

use crate::{
    config::config::SdkConfig,
    model::artifact::{
        Artifact, CreateArtifactDTO, CreateArtifactResponse, DownloadArtifactResponse,
        UpdateArtifactDTO,
    },
    utils::url::{create_path, download_path, get_path, list_path, update_path},
};

pub const ARTIFACT_PATH_ROOT: &str = "artifact";

pub async fn create(config: SdkConfig, input: CreateArtifactDTO) -> CreateArtifactResponse {
    let client = reqwest::Client::new();
    let res = client
        .post(create_path(config, ARTIFACT_PATH_ROOT))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not create artifact"),
    }
}

pub async fn get(config: SdkConfig, id: &str) -> Artifact {
    let client = reqwest::Client::new();
    let res = client
        .get(get_path(config, ARTIFACT_PATH_ROOT, id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get artifact"),
    }
}

pub async fn update(config: SdkConfig, id: &str, input: UpdateArtifactDTO) {
    let client = reqwest::Client::new();
    let res = client
        .post(update_path(config, ARTIFACT_PATH_ROOT, id))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update artifact"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Artifact> {
    let client = reqwest::Client::new();
    let res = client
        .post(list_path(config, ARTIFACT_PATH_ROOT))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not list artifacts"),
    }
}

pub async fn download(config: SdkConfig, id: &str) -> DownloadArtifactResponse {
    let client = reqwest::Client::new();
    let res = client
        .get(download_path(config, ARTIFACT_PATH_ROOT, id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get artifact"),
    }
}

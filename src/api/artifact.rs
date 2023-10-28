use log::debug;
use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::config::SdkConfig,
    model::artifact::{
        Artifact, CreateArtifactDTO, CreateArtifactResponse, DownloadArtifactResponse,
        UpdateArtifactDTO,
    },
    utils::{
        api::reqwest_client,
        auth::add_auth,
        url::{create_path, download_path, get_path, list_path, update_path},
    },
};

pub const ARTIFACT_PATH_ROOT: &str = "artifact";

pub async fn create(config: SdkConfig, input: CreateArtifactDTO) -> CreateArtifactResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(create_path(config.clone(), ARTIFACT_PATH_ROOT))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse CreateArtifactResponse"),
        Err(_) => panic!("could not create artifact"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> Artifact {
    let client = reqwest_client();
    let res = add_auth(
        client.get(get_path(
            config.clone(),
            ARTIFACT_PATH_ROOT,
            id.to_string().as_str(),
        )),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse Artifact"),
        Err(_) => panic!("could not get artifact"),
    }
}

pub async fn update(config: SdkConfig, id: ObjectId, input: UpdateArtifactDTO) {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(update_path(
                config.clone(),
                ARTIFACT_PATH_ROOT,
                id.to_string().as_str(),
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update artifact"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Artifact> {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(list_path(config.clone(), ARTIFACT_PATH_ROOT))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse Vec<Artifact>"),
        Err(_) => panic!("could not list artifacts"),
    }
}

pub async fn download(config: SdkConfig, id: ObjectId) -> DownloadArtifactResponse {
    let client = reqwest_client();
    let res = add_auth(
        client.get(download_path(
            config.clone(),
            ARTIFACT_PATH_ROOT,
            id.to_string().as_str(),
        )),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse DownloadArtifactResponse"),
        Err(_) => panic!("could not get artifact"),
    }
}

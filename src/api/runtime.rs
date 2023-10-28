use log::debug;
use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::{
        config::SdkConfig,
        constants::{API_ROUTE_PROJECT, API_ROUTE_RUNTIME, DOWNLOAD_VERB, LIST_VERB, UPDATE_VERB},
    },
    model::runtime::{
        CreateRuntimeDTO, CreateRuntimeResponse, DownloadRuntimeResponse, Runtime, UpdateRuntimeDTO,
    },
    utils::{api::reqwest_client, auth::add_auth, url::format_path_components},
};

pub async fn create(
    config: SdkConfig,
    project_id: ObjectId,
    input: CreateRuntimeDTO,
) -> CreateRuntimeResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![
                    API_ROUTE_PROJECT,
                    &project_id.to_string(),
                    API_ROUTE_RUNTIME,
                ],
            ))
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
            .expect("could not parse CreateRuntimeResponse"),
        Err(_) => panic!("could not create runtime"),
    }
}

pub async fn get(config: SdkConfig, project_id: ObjectId, id: ObjectId) -> Runtime {
    let client = reqwest_client();
    let res = add_auth(
        client.get(format_path_components(
            config.clone(),
            vec![
                API_ROUTE_PROJECT,
                &project_id.to_string(),
                API_ROUTE_RUNTIME,
                &id.to_string(),
            ],
        )),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse Runtime"),
        Err(_) => panic!("could not get runtime"),
    }
}

pub async fn update(
    config: SdkConfig,
    project_id: ObjectId,
    id: ObjectId,
    input: UpdateRuntimeDTO,
) {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![
                    API_ROUTE_PROJECT,
                    &project_id.to_string(),
                    API_ROUTE_RUNTIME,
                    &id.to_string(),
                    UPDATE_VERB,
                ],
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update runtime"),
    }
}

pub async fn list(config: SdkConfig, project_id: ObjectId, input: Document) -> Vec<Runtime> {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![
                    API_ROUTE_PROJECT,
                    &project_id.to_string(),
                    API_ROUTE_RUNTIME,
                    LIST_VERB,
                ],
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse Vec<Runtime>"),
        Err(_) => panic!("could not list runtimes"),
    }
}

pub async fn download(
    config: SdkConfig,
    project_id: ObjectId,
    id: ObjectId,
) -> DownloadRuntimeResponse {
    let client = reqwest_client();
    let res = add_auth(
        client.get(format_path_components(
            config.clone(),
            vec![
                API_ROUTE_PROJECT,
                &project_id.to_string(),
                API_ROUTE_RUNTIME,
                &id.to_string(),
                DOWNLOAD_VERB,
            ],
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
            .expect("could not parse DownloadRuntimeResponse"),
        Err(_) => panic!("could not get runtime"),
    }
}

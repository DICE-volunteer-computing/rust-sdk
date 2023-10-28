use log::debug;
use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::{
        config::SdkConfig,
        constants::{API_ROUTE_PROJECT, LIST_VERB},
    },
    model::project::{CreateProjectDTO, CreateProjectResponse, Project},
    utils::{api::reqwest_client, auth::add_auth, url::format_path_components},
};

pub async fn create(config: SdkConfig, input: CreateProjectDTO) -> CreateProjectResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![API_ROUTE_PROJECT],
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
            .expect("could not parse CreateProjectResponse"),
        Err(_) => panic!("could not create project"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> Project {
    let client = reqwest_client();
    let res = add_auth(
        client.get(format_path_components(
            config.clone(),
            vec![API_ROUTE_PROJECT, &id.to_string()],
        )),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse Project"),
        Err(_) => panic!("could not get project"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Project> {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![API_ROUTE_PROJECT, LIST_VERB],
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse Vec<Project>"),
        Err(_) => panic!("could not list projects"),
    }
}

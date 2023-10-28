use log::debug;
use mongodb::bson::{oid::ObjectId, Document};

TODO: Add new API endpoints here

use crate::{
    config::{
        config::SdkConfig,
        constants::{API_ROUTE_HOST, LIST_VERB, UPDATE_VERB},
    },
    model::host::{CreateHostDTO, CreateHostResponse, Host, UpdateHostDTO},
    utils::{api::reqwest_client, auth::add_auth, url::format_path_components},
};

pub async fn create(config: SdkConfig, input: CreateHostDTO) -> CreateHostResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(config.clone(), vec![API_ROUTE_HOST]))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => {
            let data: CreateHostResponse = response
                .json()
                .await
                .expect("could not parse CreateHostResponse");

            data
        }
        Err(_) => panic!("could not create host"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> Host {
    let client = reqwest_client();
    let res = add_auth(
        client.get(format_path_components(config.clone(), vec![API_ROUTE_HOST])),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse Host"),
        Err(_) => panic!("could not get host"),
    }
}

pub async fn update(config: SdkConfig, input: UpdateHostDTO) {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![API_ROUTE_HOST, UPDATE_VERB],
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update host"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Host> {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![API_ROUTE_HOST, LIST_VERB],
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse Vec<Host>"),
        Err(_) => panic!("could not list hosts"),
    }
}

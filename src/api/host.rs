use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::config::SdkConfig,
    model::host::{CreateHostDTO, CreateHostResponse, Host, UpdateHostDTO},
    utils::{
        api::create_client,
        auth::{add_auth, get_api_token, get_registration_token, set_api_token},
        url::{create_path, get_path, list_path, update_path},
    },
};

pub const HOST_PATH_ROOTH: &str = "host";

pub async fn create(config: SdkConfig, input: CreateHostDTO) -> CreateHostResponse {
    let client = create_client();
    let res = add_auth(
        client
            .post(create_path(config, HOST_PATH_ROOTH))
            .json(&input),
        &get_registration_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => {
            let data: CreateHostResponse = response
                .json()
                .await
                .expect("could not parse CreateHostResponse");

            set_api_token(&data.token);

            data
        }
        Err(_) => panic!("could not create host"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> Host {
    let client = reqwest::Client::new();
    let res = add_auth(
        client.get(get_path(config, HOST_PATH_ROOTH, id.to_string().as_str())),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse Host"),
        Err(_) => panic!("could not get host"),
    }
}

pub async fn update(config: SdkConfig, id: ObjectId, input: UpdateHostDTO) {
    let client = reqwest::Client::new();
    let res = add_auth(
        client
            .post(update_path(
                config,
                HOST_PATH_ROOTH,
                id.to_string().as_str(),
            ))
            .json(&input),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update host"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Host> {
    let client = reqwest::Client::new();
    let res = add_auth(
        client.post(list_path(config, HOST_PATH_ROOTH)).json(&input),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse Vec<Host>"),
        Err(_) => panic!("could not list hosts"),
    }
}

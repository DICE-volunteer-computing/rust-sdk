use mongodb::bson::Document;
use serde_json::Value;

use crate::{
    config::config::SdkConfig,
    model::host::{CreateHostDTO, Host, UpdateHostDTO},
    utils::url::{create_path, get_path, list_path, update_path},
};

pub const HOST_PATH_ROOTH: &str = "host";

pub async fn create(config: SdkConfig, input: CreateHostDTO) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post(create_path(config, HOST_PATH_ROOTH))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => serde_json::from_str::<Value>(&response.text().await.unwrap()).unwrap()
            ["insertedId"]["$oid"]
            .to_string()
            .replace("\"", ""),
        Err(_) => panic!("could not create host"),
    }
}

pub async fn get(config: SdkConfig, id: &str) -> Host {
    let client = reqwest::Client::new();
    let res = client
        .get(get_path(config, HOST_PATH_ROOTH, id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get host"),
    }
}

pub async fn update(config: SdkConfig, id: &str, input: UpdateHostDTO) {
    let client = reqwest::Client::new();
    let res = client
        .post(update_path(config, HOST_PATH_ROOTH, id))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update host"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Host> {
    let client = reqwest::Client::new();
    let res = client
        .post(list_path(config, HOST_PATH_ROOTH))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not list hosts"),
    }
}

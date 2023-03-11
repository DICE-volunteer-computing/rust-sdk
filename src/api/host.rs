use mongodb::bson::Document;
use serde_json::Value;

use crate::model::host::{CreateHostDTO, Host, UpdateHostDTO};

pub async fn create(input: CreateHostDTO) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/host")
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

pub async fn get(id: String) -> Host {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8080/host/{}", id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get host"),
    }
}

pub async fn update(id: String, input: UpdateHostDTO) {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://localhost:8080/host/{}/update", id))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update host"),
    }
}

pub async fn list(input: Document) -> Vec<Host> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/host/list")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not list hosts"),
    }
}

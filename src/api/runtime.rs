use mongodb::bson::Document;

use crate::model::runtime::{
    CreateRuntimeDTO, CreateRuntimeResponse, DownloadRuntimeResponse, Runtime, UpdateRuntimeDTO,
};

pub async fn create(input: CreateRuntimeDTO) -> CreateRuntimeResponse {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/runtime")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not create runtime"),
    }
}

pub async fn get(id: String) -> Runtime {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8080/runtime/{}", id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get runtime"),
    }
}

pub async fn update(id: String, input: UpdateRuntimeDTO) {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://localhost:8080/runtime/{}/update", id))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update runtime"),
    }
}

pub async fn list(input: Document) -> Vec<Runtime> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/runtime/list")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not list runtimes"),
    }
}

pub async fn download(id: String) -> DownloadRuntimeResponse {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8080/runtime/{}/download", id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get runtime"),
    }
}

use mongodb::bson::Document;

use crate::model::artifact::{
    Artifact, CreateArtifactDTO, CreateArtifactResponse, DownloadArtifactResponse,
    UpdateArtifactDTO,
};

pub async fn create(input: CreateArtifactDTO) -> CreateArtifactResponse {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/artifact")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not create artifact"),
    }
}

pub async fn get(id: String) -> Artifact {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8080/artifact/{}", id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get artifact"),
    }
}

pub async fn update(id: String, input: UpdateArtifactDTO) {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://localhost:8080/artifact/{}/update", id))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update artifact"),
    }
}

pub async fn list(input: Document) -> Vec<Artifact> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/artifact/list")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not list artifacts"),
    }
}

pub async fn download(id: String) -> DownloadArtifactResponse {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8080/artifact/{}/download", id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get artifact"),
    }
}

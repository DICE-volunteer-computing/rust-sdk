use mongodb::bson::Document;
use serde_json::Value;

use crate::model::project::{CreateProjectDTO, Project};

pub async fn create(input: CreateProjectDTO) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/project")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => serde_json::from_str::<Value>(&response.text().await.unwrap()).unwrap()
            ["insertedId"]["$oid"]
            .to_string()
            .replace("\"", ""),
        Err(_) => panic!("could not create project"),
    }
}

pub async fn get(id: String) -> Project {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8080/project/{}", id))
        .send()
        .await;

    match res {
        Ok(response) => serde_json::from_str(&response.text().await.unwrap()).unwrap(),
        Err(_) => panic!("could not get project"),
    }
}

pub async fn list(input: Document) -> Vec<Project> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/project/list")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => serde_json::from_str(&response.text().await.unwrap()).unwrap(),
        Err(_) => panic!("could not list projects"),
    }
}

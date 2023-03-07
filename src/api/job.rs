use mongodb::bson::Document;

use crate::model::job::{CreateJobDTO, CreateJobResponse, Job};

pub async fn create(input: CreateJobDTO) -> CreateJobResponse {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/job")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not create job"),
    }
}

pub async fn get(id: String) -> Job {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8080/job/{}", id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get job"),
    }
}

pub async fn list(input: Document) -> Vec<Job> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/job/list")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not list jobs"),
    }
}

use mongodb::bson::Document;

use crate::model::job_execution::{
    CreateJobExecutionDTO, CreateJobExecutionResponse, JobExecution,
};

pub async fn create(input: CreateJobExecutionDTO) -> CreateJobExecutionResponse {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/job_execution")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not create job execution"),
    }
}

pub async fn get(id: String) -> JobExecution {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:8080/job_execution/{}", id))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not get job execution"),
    }
}

pub async fn list(input: Document) -> Vec<JobExecution> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/job_execution/list")
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(_) => panic!("could not list job executions"),
    }
}

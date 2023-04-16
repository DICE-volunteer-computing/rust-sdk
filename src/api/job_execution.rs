use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::config::SdkConfig,
    model::job_execution::{
        CreateJobExecutionDTO, CreateJobExecutionResponse, JobExecution, UpdateJobExecutionDTO,
    },
    utils::url::{create_path, get_path, list_path, update_path},
};

pub const JOB_EXECUTION_PATH_ROOT: &str = "job_execution";

pub async fn create(config: SdkConfig, input: CreateJobExecutionDTO) -> CreateJobExecutionResponse {
    let client = reqwest::Client::new();
    let res = client
        .post(create_path(config, JOB_EXECUTION_PATH_ROOT))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse CreateJobExecutionResponse"),
        Err(_) => panic!("could not create job execution"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> JobExecution {
    let client = reqwest::Client::new();
    let res = client
        .get(get_path(
            config,
            JOB_EXECUTION_PATH_ROOT,
            id.to_string().as_str(),
        ))
        .send()
        .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse JobExecution"),
        Err(_) => panic!("could not get job execution"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<JobExecution> {
    let client = reqwest::Client::new();
    let res = client
        .post(list_path(config, JOB_EXECUTION_PATH_ROOT))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse Vec<JobExecution>"),
        Err(_) => panic!("could not list job executions"),
    }
}

pub async fn update(config: SdkConfig, id: ObjectId, input: UpdateJobExecutionDTO) {
    let client = reqwest::Client::new();
    let res = client
        .post(update_path(
            config,
            JOB_EXECUTION_PATH_ROOT,
            id.to_string().as_str(),
        ))
        .json(&input)
        .send()
        .await;

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update job execution"),
    }
}

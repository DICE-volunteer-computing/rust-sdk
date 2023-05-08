use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::config::SdkConfig,
    model::job::{CreateJobDTO, CreateJobResponse, Job},
    utils::{
        auth::{add_auth, get_api_token},
        url::{create_path, get_path, list_path},
    },
};

pub const JOB_PATH_ROOT: &str = "job";

pub async fn create(config: SdkConfig, input: CreateJobDTO) -> CreateJobResponse {
    let client = reqwest::Client::new();
    let res = add_auth(
        client.post(create_path(config, JOB_PATH_ROOT)).json(&input),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse CreateJobResponse"),
        Err(_) => panic!("could not create job"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> Job {
    let client = reqwest::Client::new();
    let res = add_auth(
        client.get(get_path(config, JOB_PATH_ROOT, id.to_string().as_str())),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse Job"),
        Err(_) => panic!("could not get job"),
    }
}

pub async fn list(config: SdkConfig, input: Document) -> Vec<Job> {
    let client = reqwest::Client::new();
    let res = add_auth(
        client.post(list_path(config, JOB_PATH_ROOT)).json(&input),
        &get_api_token(),
    )
    .send()
    .await;

    match res {
        Ok(response) => response.json().await.expect("could not parse Vec<Job>"),
        Err(_) => panic!("could not list jobs"),
    }
}

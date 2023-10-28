use log::debug;
use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::{
        config::SdkConfig,
        constants::{
            API_ROUTE_JOB_EXECUTION, API_ROUTE_PROJECT, DOWNLOAD_VERB, LIST_VERB, UPDATE_VERB,
        },
    },
    model::job_execution::{
        CreateJobExecutionDTO, CreateJobExecutionResponse, DownloadJobExecutionResponse,
        JobExecution, UpdateJobExecutionDTO,
    },
    utils::{api::reqwest_client, auth::add_auth, url::format_path_components},
};

pub async fn create(
    config: SdkConfig,
    project_id: ObjectId,
    input: CreateJobExecutionDTO,
) -> CreateJobExecutionResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![
                    API_ROUTE_PROJECT,
                    &project_id.to_string(),
                    API_ROUTE_JOB_EXECUTION,
                ],
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse CreateJobExecutionResponse"),
        Err(_) => panic!("could not create job execution"),
    }
}

pub async fn get(config: SdkConfig, project_id: ObjectId, id: ObjectId) -> JobExecution {
    let client = reqwest_client();
    let res = add_auth(
        client.get(format_path_components(
            config.clone(),
            vec![
                API_ROUTE_PROJECT,
                &project_id.to_string(),
                API_ROUTE_JOB_EXECUTION,
                &id.to_string(),
            ],
        )),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse JobExecution"),
        Err(_) => panic!("could not get job execution"),
    }
}

pub async fn list(config: SdkConfig, project_id: ObjectId, input: Document) -> Vec<JobExecution> {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![
                    API_ROUTE_PROJECT,
                    &project_id.to_string(),
                    API_ROUTE_JOB_EXECUTION,
                    LIST_VERB,
                ],
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse Vec<JobExecution>"),
        Err(_) => panic!("could not list job executions"),
    }
}

pub async fn update(
    config: SdkConfig,
    project_id: ObjectId,
    id: ObjectId,
    input: UpdateJobExecutionDTO,
) {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![
                    API_ROUTE_PROJECT,
                    &project_id.to_string(),
                    API_ROUTE_JOB_EXECUTION,
                    &id.to_string(),
                    UPDATE_VERB,
                ],
            ))
            .json(&input),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(_) => (),
        Err(_) => panic!("could not update job execution"),
    }
}

pub async fn download(
    config: SdkConfig,
    project_id: ObjectId,
    id: ObjectId,
) -> DownloadJobExecutionResponse {
    let client = reqwest_client();
    let res = add_auth(
        client.get(format_path_components(
            config.clone(),
            vec![
                API_ROUTE_PROJECT,
                &project_id.to_string(),
                API_ROUTE_JOB_EXECUTION,
                &id.to_string(),
                DOWNLOAD_VERB,
            ],
        )),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response
            .json()
            .await
            .expect("could not parse DownloadJobExecutionResponse"),
        Err(_) => panic!("could not download job execution"),
    }
}

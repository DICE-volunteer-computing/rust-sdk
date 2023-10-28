use log::debug;
use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    config::{
        config::SdkConfig,
        constants::{API_ROUTE_JOB, API_ROUTE_PROJECT, LIST_VERB},
    },
    model::job::{CreateJobDTO, CreateJobResponse, Job},
    utils::{api::reqwest_client, auth::add_auth, url::format_path_components},
};

pub async fn create(
    config: SdkConfig,
    project_id: ObjectId,
    input: CreateJobDTO,
) -> CreateJobResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![API_ROUTE_PROJECT, &project_id.to_string(), API_ROUTE_JOB],
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
            .expect("could not parse CreateJobResponse"),
        Err(_) => panic!("could not create job"),
    }
}

pub async fn get(config: SdkConfig, project_id: ObjectId, id: ObjectId) -> Job {
    let client = reqwest_client();
    let res = add_auth(
        client.get(format_path_components(
            config.clone(),
            vec![
                API_ROUTE_PROJECT,
                &project_id.to_string(),
                API_ROUTE_JOB,
                &id.to_string(),
            ],
        )),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse Job"),
        Err(_) => panic!("could not get job"),
    }
}

pub async fn list(config: SdkConfig, project_id: ObjectId, input: Document) -> Vec<Job> {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(
                config.clone(),
                vec![
                    API_ROUTE_PROJECT,
                    &project_id.to_string(),
                    API_ROUTE_JOB,
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
        Ok(response) => response.json().await.expect("could not parse Vec<Job>"),
        Err(_) => panic!("could not list jobs"),
    }
}

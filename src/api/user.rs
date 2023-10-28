use log::debug;
use mongodb::bson::oid::ObjectId;

use crate::{
    config::{config::SdkConfig, constants::API_ROUTE_USER},
    model::user::{CreateUserDTO, CreateUserResponse, User},
    utils::{api::reqwest_client, auth::add_auth, url::format_path_components},
};

pub async fn create(config: SdkConfig, input: CreateUserDTO) -> CreateUserResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(format_path_components(config.clone(), vec![API_ROUTE_USER]))
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
            .expect("could not parse CreateUserResponse"),
        Err(_) => panic!("could not create user"),
    }
}

pub async fn get(config: SdkConfig, id: ObjectId) -> User {
    let client = reqwest_client();
    let res = add_auth(
        client.get(format_path_components(config.clone(), vec![API_ROUTE_USER])),
        &config.auth,
    )
    .send()
    .await;

    debug!("{:?}", res);

    match res {
        Ok(response) => response.json().await.expect("could not parse User"),
        Err(_) => panic!("could not get user"),
    }
}

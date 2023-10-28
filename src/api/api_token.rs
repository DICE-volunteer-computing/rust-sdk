use log::debug;

use crate::{
    config::config::SdkConfig,
    model::token::{CreateApiTokenDTO, CreateTokenResponse},
    utils::{api::reqwest_client, auth::add_auth, url::create_path},
};

pub const API_TOKEN_PATH_ROOT: &str = "api_token";

pub async fn create(config: SdkConfig, input: CreateApiTokenDTO) -> CreateTokenResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(create_path(config.clone(), API_TOKEN_PATH_ROOT))
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
            .expect("could not parse CreateTokenResponse"),
        Err(_) => panic!("could not create token"),
    }
}

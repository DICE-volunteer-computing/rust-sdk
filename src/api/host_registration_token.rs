use log::debug;

use crate::{
    config::config::SdkConfig,
    model::token::{CreateHostRegistrationTokenDTO, CreateTokenResponse},
    utils::{api::reqwest_client, auth::add_auth, url::create_path},
};

pub const HOST_REGISTRATION_TOKEN_PATH_ROOT: &str = "host_registration_token";

pub async fn create(
    config: SdkConfig,
    input: CreateHostRegistrationTokenDTO,
) -> CreateTokenResponse {
    let client = reqwest_client();
    let res = add_auth(
        client
            .post(create_path(
                config.clone(),
                HOST_REGISTRATION_TOKEN_PATH_ROOT,
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
            .expect("could not parse CreateTokenResponse"),
        Err(_) => panic!("could not create token"),
    }
}

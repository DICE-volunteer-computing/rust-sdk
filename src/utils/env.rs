pub const API_TOKEN_ENV_NAME: &str = "DICE_API_TOKEN";
pub const HOST_REGISTRATION_TOKEN_ENV_NAME: &str = "DICE_REGISTRATION_TOKEN";

pub fn get_api_token() -> String {
    std::env::var(API_TOKEN_ENV_NAME).expect("could not get DICE_API_TOKEN")
}

pub fn set_api_token(token: &str) {
    std::env::set_var(API_TOKEN_ENV_NAME, token);
}

pub fn get_registration_token() -> String {
    std::env::var(HOST_REGISTRATION_TOKEN_ENV_NAME).expect("could not get DICE_REGISTRATION_TOKEN")
}

pub fn set_registration_token(token: &str) {
    std::env::set_var(HOST_REGISTRATION_TOKEN_ENV_NAME, token);
}

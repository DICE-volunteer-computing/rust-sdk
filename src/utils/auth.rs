use reqwest::RequestBuilder;

pub fn get_api_token() -> String {
    std::env::var("DICE_API_TOKEN").expect("could not get DICE_API_TOKEN")
}

pub fn set_api_token(token: &str) {
    std::env::set_var("DICE_API_TOKEN", token);
}

pub fn get_registration_token() -> String {
    std::env::var("DICE_REGISTRATION_TOKEN").expect("could not get DICE_REGISTRATION_TOKEN")
}

pub fn add_auth(builder: RequestBuilder, token: &str) -> RequestBuilder {
    builder.header("Authorization", "Basic ".to_owned() + token)
}

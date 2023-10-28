use reqwest::RequestBuilder;

pub fn add_auth(builder: RequestBuilder, token: &str) -> RequestBuilder {
    builder.header("Authorization", "Basic ".to_owned() + token)
}

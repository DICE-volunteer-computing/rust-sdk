use reqwest::Client;

pub fn reqwest_client() -> Client {
    reqwest::Client::builder()
        .http1_title_case_headers()
        .build()
        .expect("could not create client")
}

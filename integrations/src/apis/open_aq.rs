use crate::client::ApiClient;

pub struct OpenAqApi {
    client: ApiClient,
    token: String,
}

impl OpenAqApi {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            client: ApiClient::new(60, 1, std::time::Duration::from_secs(2)),
            token: token.into(),
        }
    }
}

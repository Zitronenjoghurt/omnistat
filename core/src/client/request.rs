use crate::error::OmnistatResult;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use serde::de::DeserializeOwned;

pub struct RequestBuilder<'a> {
    client: &'a ClientWithMiddleware,
    url: Url,
    headers: HeaderMap,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(
        client: &'a ClientWithMiddleware,
        url: impl AsRef<str>,
    ) -> OmnistatResult<RequestBuilder<'a>> {
        Ok(Self {
            client,
            url: Url::parse(url.as_ref())?,
            headers: HeaderMap::new(),
        })
    }

    pub fn header(
        mut self,
        key: impl IntoHeaderName,
        value: impl AsRef<str>,
    ) -> OmnistatResult<Self> {
        let header_value: HeaderValue = value.as_ref().parse()?;
        self.headers.insert(key, header_value);
        Ok(self)
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(&key.into(), &value.into());
        self
    }

    pub fn path(mut self, path: impl AsRef<str>) -> Self {
        self.url.set_path(path.as_ref());
        self
    }

    pub async fn get_json<T: DeserializeOwned>(self) -> OmnistatResult<T> {
        Ok(self
            .client
            .get(self.url)
            .headers(self.headers)
            .send()
            .await?
            .json()
            .await?)
    }
}

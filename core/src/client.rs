use crate::error::OmnistatResult;
use reqwest_leaky_bucket::leaky_bucket::RateLimiter;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::time::Duration;

mod request;

pub struct ApiClient {
    client: ClientWithMiddleware,
}

impl ApiClient {
    pub fn new(tokens: usize, refill: usize, interval: Duration) -> Self {
        let limiter = RateLimiter::builder()
            .max(tokens)
            .initial(tokens)
            .refill(refill)
            .interval(interval)
            .build();
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

        let client = ClientBuilder::new(reqwest::Client::new())
            .with(reqwest_leaky_bucket::rate_limit_all(limiter))
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        Self { client }
    }

    pub fn request(
        &'_ self,
        base_url: impl AsRef<str>,
    ) -> OmnistatResult<request::RequestBuilder<'_>> {
        request::RequestBuilder::new(&self.client, base_url)
    }
}

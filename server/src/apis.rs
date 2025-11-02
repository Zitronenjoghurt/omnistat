use omnistat_core::apis::open_meteo::OpenMeteoApi;
use std::sync::Arc;

#[derive(Clone)]
pub struct Apis {
    pub open_meteo: Arc<OpenMeteoApi>,
}

impl Apis {
    pub fn initialize() -> anyhow::Result<Self> {
        Ok(Self {
            open_meteo: Arc::new(OpenMeteoApi::new()),
        })
    }
}

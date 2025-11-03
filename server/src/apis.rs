use omnistat_integrations::apis::open_meteo::OpenMeteoApi;
use std::sync::Arc;

pub struct Apis {
    pub open_meteo: Arc<OpenMeteoApi>,
}

impl Apis {
    pub fn initialize() -> anyhow::Result<Arc<Self>> {
        Ok(Arc::new(Self {
            open_meteo: Arc::new(OpenMeteoApi::new()),
        }))
    }
}

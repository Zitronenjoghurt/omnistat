use crate::apis::Apis;
use crate::config::Config;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod weather;

pub struct ServiceInitContext {
    pub config: Arc<Config>,
    pub db: Arc<DatabaseConnection>,
    pub apis: Arc<Apis>,
}

#[derive(Clone)]
pub struct Services {
    pub weather: Arc<weather::WeatherService>,
}

impl Services {
    pub fn initialize(context: ServiceInitContext) -> Arc<Self> {
        Arc::new(Self {
            weather: weather::WeatherService::initialize(&context),
        })
    }
}

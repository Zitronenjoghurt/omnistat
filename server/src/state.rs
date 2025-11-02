use crate::apis::Apis;
use crate::config::Config;
use crate::services::{ServiceInitContext, Services};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    pub config: Arc<Config>,
    pub db: Arc<DatabaseConnection>,
    pub apis: Arc<Apis>,
    pub services: Arc<Services>,
}

impl ServerState {
    pub async fn initialize() -> anyhow::Result<Arc<Self>> {
        let config = Config::load_from_env()?;
        let db = Self::initialize_db(&config).await?;
        let apis = Apis::initialize()?;

        let service_init_context = ServiceInitContext {
            config: config.clone(),
            db: db.clone(),
            apis: apis.clone(),
        };
        let services = Services::initialize(service_init_context);

        Ok(Arc::new(Self {
            config,
            db,
            apis,
            services,
        }))
    }

    async fn initialize_db(config: &Arc<Config>) -> anyhow::Result<Arc<DatabaseConnection>> {
        let options = ConnectOptions::new(&config.db_url);
        let connection = sea_orm::Database::connect(options).await?;
        Migrator::up(&connection, None).await?;
        Ok(Arc::new(connection))
    }
}

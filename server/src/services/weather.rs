use crate::config::Config;
use crate::database::entity::hourly_weather;
use crate::services::ServiceInitContext;
use omnistat_integrations::apis::open_meteo::hourly_forecast::OpenMeteoHourly;
use omnistat_integrations::apis::open_meteo::OpenMeteoApi;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::{error, info};

mod hour;

#[derive(Clone)]
pub struct WeatherService {
    config: Arc<Config>,
    db: Arc<DatabaseConnection>,
    open_meteo: Arc<OpenMeteoApi>,
}

impl WeatherService {
    pub fn initialize(context: &ServiceInitContext) -> Arc<Self> {
        Arc::new(Self {
            config: context.config.clone(),
            db: context.db.clone(),
            open_meteo: context.apis.open_meteo.clone(),
        })
    }

    pub async fn sync_hourly_weather(&self) {
        info!("Syncing hourly weather...");
        for user_id in self.config.users.keys() {
            info!("Processing user {}", user_id);
            let result = self.sync_hourly_weather_user(user_id).await;
            if let Err(e) = result {
                error!(
                    "Failed to sync hourly weather for user '{}': {}",
                    user_id, e
                );
            } else {
                info!("Successfully processed user {}", user_id);
            }
        }
        info!("Finished syncing hourly weather");
    }

    async fn sync_hourly_weather_user(&self, user_id: &str) -> anyhow::Result<()> {
        let config_user = self.config.get_user_or_err(user_id)?;
        let open_meteo_hourlies = self
            .open_meteo
            .hourly_forecasts(config_user.latitude, config_user.longitude)
            .await?;
        for hourly in open_meteo_hourlies {
            self.sync_open_meteo_hourly(user_id, hourly).await?;
        }
        Ok(())
    }

    async fn sync_open_meteo_hourly(
        &self,
        user_id: &str,
        hourly: OpenMeteoHourly,
    ) -> anyhow::Result<()> {
        let active_model = hourly_weather::ActiveModel::from_open_meteo(&hourly, user_id);
        hourly_weather::Entity::upsert(active_model, self.db.as_ref()).await?;
        info!(
            "Synced open meteo hourly of '{}' for user '{}'",
            hourly.time, user_id
        );
        Ok(())
    }
}

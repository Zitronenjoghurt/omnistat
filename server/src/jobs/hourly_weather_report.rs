use crate::config::user::ConfigUser;
use crate::database::entity::hourly_weather;
use crate::state::ServerState;
use omnistat_core::apis::open_meteo::hourly_forecast::HourlyForecast;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

pub async fn job_hourly_weather_report(
    scheduler: &JobScheduler,
    state: Arc<ServerState>,
) -> anyhow::Result<()> {
    let job = Job::new_async("0 5 * * * *", move |_uuid, _l| {
        let state = state.clone();
        Box::pin(async move {
            if let Err(e) = hourly_weather_report(state).await {
                eprintln!("Hourly weather report failed: {}", e);
            }
        })
    })?;
    scheduler.add(job).await?;
    Ok(())
}

async fn hourly_weather_report(state: Arc<ServerState>) -> anyhow::Result<()> {
    info!("Starting hourly weather report...");

    for (user_id, user) in state.config.users.iter() {
        info!("Processing user {}", user_id);
        hourly_weather_report_user(state.clone(), user_id, user).await?;
        info!("Successfully processed user {}", user_id);
    }

    info!("Finished hourly weather report");
    Ok(())
}

async fn hourly_weather_report_user(
    state: Arc<ServerState>,
    user_id: &str,
    user: &ConfigUser,
) -> anyhow::Result<()> {
    let hourly_forecasts = state
        .apis
        .open_meteo
        .hourly_forecasts(user.latitude, user.longitude)
        .await?;

    for forecast in hourly_forecasts {
        sync_hourly_weather(state.clone(), user_id, forecast).await?;
    }

    Ok(())
}

async fn sync_hourly_weather(
    state: Arc<ServerState>,
    user_id: &str,
    forecast: HourlyForecast,
) -> anyhow::Result<()> {
    let active_model = hourly_weather::ActiveModel::from_forecast(&forecast, user_id);
    hourly_weather::Entity::upsert(active_model, state.db.as_ref()).await?;
    info!(
        "Synced forecast from {} for user {}",
        forecast.time, user_id
    );
    Ok(())
}

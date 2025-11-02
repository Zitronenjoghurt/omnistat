use crate::state::ServerState;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn job_hourly_weather_report(
    scheduler: &JobScheduler,
    state: Arc<ServerState>,
) -> anyhow::Result<()> {
    let job = Job::new_async("0 5 * * * *", move |_uuid, _l| {
        let state = state.clone();
        Box::pin(async move {
            state.services.weather.sync_hourly_weather().await;
        })
    })?;
    scheduler.add(job).await?;
    Ok(())
}

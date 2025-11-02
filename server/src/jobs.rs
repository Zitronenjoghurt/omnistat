use crate::state::ServerState;
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;

mod hourly_weather_report;

pub async fn start_jobs(state: Arc<ServerState>) -> anyhow::Result<()> {
    let scheduler = JobScheduler::new().await?;
    hourly_weather_report::job_hourly_weather_report(&scheduler, state).await?;
    scheduler.start().await?;
    Ok(())
}

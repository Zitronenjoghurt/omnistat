use crate::jobs::start_jobs;
use crate::state::ServerState;
use tracing::info;

mod apis;
mod config;
mod database;
mod jobs;
mod state;

#[tokio::main]
async fn main() {
    init_tracing();

    let state = ServerState::initialize().await.unwrap();
    info!("Initialized state");

    start_jobs(state.clone()).await.unwrap();
    info!("Started jobs");

    info!("Server started");

    tokio::signal::ctrl_c().await.unwrap();
    info!("Shutting down...");
}

fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                EnvFilter::new("debug")
            } else {
                EnvFilter::new("info")
            }
        }))
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();
}

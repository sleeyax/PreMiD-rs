use tracing::{metadata::LevelFilter, subscriber::SetGlobalDefaultError};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn setup_logger() -> Result<(), SetGlobalDefaultError> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::OFF.into())
        .from_env_lossy();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
}

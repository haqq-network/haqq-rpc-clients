use tracing_subscriber::{filter::LevelFilter, EnvFilter};

pub fn init() {
    // color_eyre::install().ok();
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()
                .unwrap(),
        )
        .try_init()
        .ok();
}

use tracing_subscriber::EnvFilter;

pub fn init() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")))
        .with_file(true)
        .with_line_number(true)
        .init();
}

pub fn setup() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // TODO: made configurable
        .with_target(false)
        .try_init()
        .expect("Failed to set up global logger");
}

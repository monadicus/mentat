//! This modules contains logging init and clean up.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

// TODO: @mowkoshka move to node conf.

/// Sets up logging for Mentat.
pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let tracer =
        opentelemetry_jaeger::new_pipeline().install_batch(opentelemetry::runtime::Tokio)?;
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    Registry::default()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug,tower_http=debug".to_string()),
        ))
        .with(
            HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .with(tracing_error::ErrorLayer::default())
        .with(telemetry)
        .init();

    Ok(())
}

/// Tears down logging for Mentat.
pub(crate) fn teardown() {
    opentelemetry::global::shutdown_tracer_provider();
}

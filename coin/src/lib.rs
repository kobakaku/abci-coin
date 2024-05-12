use std::{env, str::FromStr};

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn initialize_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::from_str(&env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string()))
                .unwrap(),
        )
        .init();
}

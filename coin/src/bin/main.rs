use coin::initialize_logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_logging();

    tracing::info!("Starting an ABCI application.");

    Ok(())
}

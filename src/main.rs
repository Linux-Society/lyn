use logging::setup_logging;
use tracing::info;

mod config;
mod logging;

fn main() -> anyhow::Result<()> {
    // Logging subscriber initialisation (stderr, file, journald)
    setup_logging();
    info!("Lyn now starting");

    Ok(())
}

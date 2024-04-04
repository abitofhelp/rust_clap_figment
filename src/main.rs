use config::Config;

mod cli;
mod config;

/// This is the entry point for the application.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build()?;
    dbg!(config);
    Ok(())
}

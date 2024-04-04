use std::error::Error;
use std::str::FromStr;

use clap::Parser;
use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use humantime_serde::re::humantime;
use serde::{Deserialize, Serialize};

use crate::cli::Cli;

/// The default connection timeout when the value has not been set elsewhere.
const DEFAULT_CONNECTION_TIMEOUT: &str = "10s";

/// The default name when the value has not been set elsewhere.
const DEFAULT_NAME: &str = "Joe";

/// The prefix used to group environment variables for use by the application.
const ENVVAR_PREFIX: &str = "RCF__";

/// The global configuration for the application
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    /// config_path is the path to a configuration (.toml) file.  The default action is to look for config.toml is the current directory where the application is executing.
    pub(crate) config_path: std::path::PathBuf,

    /// The name of the client
    #[serde(default = "default_name")]
    pub(crate) name: String,

    /// The connection timeout
    #[serde(default = "default_duration")]
    #[serde(with = "humantime_serde")]
    pub(crate) connection_timeout: std::time::Duration,
}

// The following functions, prefixed by 'default_', are used to set default values.
// If the default values are set in struct Cli, they will always override values
// from the config file/struct Config. This opposes our goal to prioritize
// arguments: defaults < file < envvar < cli.
/// Returns the default value for the name of a client when it has not been set elsewhere.  N.B. defaults < file < envvar < cli
fn default_name() -> String {
    DEFAULT_NAME.parse().unwrap()
}

/// Returns the default value for the connection timeout when it has not been set elsewhere.  N.B. defaults < file < envvar < cli
fn default_duration() -> std::time::Duration {
    humantime::Duration::from_str(DEFAULT_CONNECTION_TIMEOUT)
        .unwrap()
        .into()
}

impl Config {
    /// Build the application configuration by merging Clap's arguments with other configuration sources.
    pub(crate) fn build() -> Result<Self, Box<dyn Error>> {
        // Hierarchical Configuration: defaults < file < envvar < cli

        let cli = Cli::parse();
        let cp = cli.config_path.as_path();

        // Hierarchical configuration: 'defaults < file < envvar < cli'
        // So: cli args override Envars which override toml config values which override default values.
        let config: Config = Figment::new()
            .merge(Toml::file(cp))
            .merge(Env::prefixed(ENVVAR_PREFIX))
            .merge(Serialized::defaults(&cli))
            .extract()?;

        Ok(config)
    }
}

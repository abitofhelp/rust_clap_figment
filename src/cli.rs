use clap::Parser;
use humantime_serde::re::humantime;
use serde::Serialize;

/// The definition of the command line and its arguments
#[derive(Debug, Parser, Serialize)]
#[command(author, version, about, long_about = None, next_line_help = true)]
pub(crate) struct Cli {
    /// config_path is the path to a configuration (.toml) file.  The default action is to look for config.toml is the current directory where the application is executing.
    #[arg(short = 'c', long, default_value = "config.toml")]
    pub(crate) config_path: std::path::PathBuf,

    /// The name of a client
    #[arg(short = 'n', long)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub(crate) name: Option<String>,

    /// The connection timeout
    #[arg(short = 'd', long, value_parser = humantime::parse_duration)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(with = "humantime_serde")]
    pub(crate) connection_timeout: Option<std::time::Duration>,
}

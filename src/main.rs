use std::{error::Error, fs};

use bloginator::{Cli, Commands, FileConfig, LogLevel, set_log_level};
use clap::Parser;
use tracing::{Level, event, span};

fn main() -> Result<(), Box<dyn Error>> {
	let cli = Cli::parse();

	if cli.verbose {
		set_log_level(LogLevel::Verbose);
		event!(Level::TRACE, "set log level to verbose due to CLI flag");
	};

	let config_span = span!(Level::TRACE, "read_config_file");

	let file_config = {
		let _enter = config_span.enter();
		match fs::read_to_string(&cli.config) {
			Ok(contents) => {
				event!(Level::TRACE, "successfully read config file");
				
				let config: FileConfig = match toml::from_str(&contents) {
					Ok(config) => {
						event!(Level::TRACE, "successfully parsed config file");
						config
					},
					Err(e) => {
						event!(Level::DEBUG, "failed to parse config file: {e}");
						event!(Level::INFO, "using default config file configuration");
						FileConfig::default()
					}
				};

				config
			},
			Err(e) => {
				event!(Level::DEBUG, config_file_location = cli.config.display().to_string(), "failed to read config file: {e}");
				event!(Level::INFO, "using default config file configuration");
				FileConfig::default()
			}
		}
	};

	let config = file_config.overlay_cli_options(&cli);

	// If cli.verbose is true, then the log level has already been set and would panic if set again
	match (config.verbose, cli.verbose) {
		(true, false) => {
			set_log_level(LogLevel::Verbose);
			event!(Level::TRACE, "set log level to verbose due to config file");
		},
		(false, false) => {
			set_log_level(LogLevel::Simple);
			event!(Level::TRACE, "set log level to simple due to config file");
		},
		(_, _) => ()
	}

	event!(Level::TRACE, "config ready: {config:?}");

	match cli.command {
		Commands::Build { .. } => {
			event!(Level::TRACE, "build command supplied");
		}
	}

	Ok(())
}

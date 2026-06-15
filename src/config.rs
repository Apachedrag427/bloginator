use std::path::PathBuf;

use super::cli::{Cli, Commands};

use serde_derive::Deserialize;

// No config location is specified in FileConfig or Config because, at this point, the config file has already been either discovered or defaulted to all None
#[derive(Deserialize, Default)]
pub struct FileConfig {
	pub build: Option<FileBuildConfig>,
	pub verbose: Option<bool>,
}

#[derive(Deserialize, Default)]
pub struct FileBuildConfig {
	pub output_folder: Option<PathBuf>,
}

impl FileConfig {
	pub fn overlay_cli_options(&self, options: &Cli) -> Config {
		Config {
			// If either verbose is specified, then we're in verbose mode
			verbose: self.verbose.unwrap_or(false) | options.verbose,
			build_config: match &options.command {
				Commands::Build {
					output_folder: Some(output_folder),
				} => BuildConfig {
					output_folder: output_folder.clone(),
				},
				_ => match &self.build {
					Some(FileBuildConfig {
						output_folder: Some(output_folder),
					}) => BuildConfig {
						output_folder: output_folder.clone(),
					},
					_ => BuildConfig::default(),
				},
			},
		}
	}
}

#[derive(Debug)]
pub struct Config {
	pub build_config: BuildConfig,
	pub verbose: bool,
}

#[derive(Debug)]
pub struct BuildConfig {
	pub output_folder: PathBuf,
}

impl Default for BuildConfig {
	fn default() -> Self {
		BuildConfig {
			output_folder: "./build".into(),
		}
	}
}

mod test {
	use super::*;

	#[allow(unused)]
	fn empty_cli() -> Cli {
		Cli {
			command: Commands::Build {
				output_folder: None,
			},
			config: "./bloginator.toml".into(),
			verbose: false,
		}
	}

	#[test]
	fn all_unspecified() {
		let empty_fileconfig = FileConfig::default();
		let empty_cli_options = empty_cli();

		let config = empty_fileconfig.overlay_cli_options(&empty_cli_options);

		assert_eq!(config.build_config.output_folder, PathBuf::from("./build"));
		assert_eq!(config.verbose, false);
	}

	#[test]
	fn fileconfig_unspecified() {
		let empty_fileconfig = FileConfig::default();
		let full_cli_options = Cli {
			command: Commands::Build {
				output_folder: Some("./testbuild".into()),
			},
			config: "./testconfig.toml".into(),
			verbose: true,
		};

		let config = empty_fileconfig.overlay_cli_options(&full_cli_options);

		assert_eq!(
			config.build_config.output_folder,
			PathBuf::from("./testbuild")
		);
		assert_eq!(config.verbose, true);
	}

	#[test]
	fn cli_unspecified() {
		let full_fileconfig = FileConfig {
			build: Some(FileBuildConfig {
				output_folder: Some("./testbuild".into()),
			}),
			verbose: Some(true),
		};
		let empty_cli_options = empty_cli();

		let config = full_fileconfig.overlay_cli_options(&empty_cli_options);

		assert_eq!(
			config.build_config.output_folder,
			PathBuf::from("./testbuild")
		);
		assert_eq!(config.verbose, true);
	}

	// The difference between all_specified v1 and v2 is that
	// in v1, the fileconfig has verbose as false and the cli has verbose as true, but
	// in v2, it is reversed
	#[test]
	fn all_specified_v1() {
		let full_fileconfig = FileConfig {
			build: Some(FileBuildConfig {
				output_folder: Some("./testbuild1".into()),
			}),
			verbose: Some(false),
		};
		let full_cli_options = Cli {
			command: Commands::Build {
				output_folder: Some("./testbuild2".into()),
			},
			config: "./testconfig.toml".into(),
			verbose: true,
		};

		let config = full_fileconfig.overlay_cli_options(&full_cli_options);

		assert_eq!(
			config.build_config.output_folder,
			PathBuf::from("./testbuild2")
		);
		assert_eq!(config.verbose, true);
	}

	#[test]
	fn all_specified_v2() {
		let full_fileconfig = FileConfig {
			build: Some(FileBuildConfig {
				output_folder: Some("./testbuild1".into()),
			}),
			verbose: Some(true),
		};
		let full_cli_options = Cli {
			command: Commands::Build {
				output_folder: Some("./testbuild2".into()),
			},
			config: "./testconfig.toml".into(),
			verbose: false,
		};

		let config = full_fileconfig.overlay_cli_options(&full_cli_options);

		assert_eq!(
			config.build_config.output_folder,
			PathBuf::from("./testbuild2")
		);
		assert_eq!(config.verbose, true);
	}
}

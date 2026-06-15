use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bloginator")]
#[command(about = "CLI for simple static blog pages", long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,

	/// Specifies the config file's location
	#[arg(short, long, default_value = "./bloginator.toml", value_name = "FILE")]
	pub config: PathBuf,

	/// Outputs additional debug and trace information
	#[arg(short, long)]
	pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
	Build {
		/// Determines the folder that the built website structure will be put into
		#[arg(short, long, value_name = "DIRECTORY")]
		output_folder: Option<PathBuf>,
	},
}

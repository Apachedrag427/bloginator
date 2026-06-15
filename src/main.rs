use std::{error::Error, fs, io::Write};

use bloginator::{Cli, Commands, FileConfig, LogLevel, fsutils::*, set_log_level};
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
					}
					Err(e) => {
						event!(Level::DEBUG, "failed to parse config file: {e}");
						event!(Level::INFO, "using default config file configuration");
						FileConfig::default()
					}
				};

				config
			}
			Err(e) => {
				event!(
					Level::DEBUG,
					config_file_location = cli.config.display().to_string(),
					"failed to read config file: {e}"
				);
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
		}
		(false, false) => {
			set_log_level(LogLevel::Simple);
			event!(Level::TRACE, "set log level to simple due to config file");
		}
		(_, _) => (),
	}

	event!(Level::DEBUG, "config ready: {config:?}");

	// Cli command settings are excluded because config is the new source of truth
	match cli.command {
		Commands::Build { .. } => {
			event!(Level::TRACE, "build command supplied");

			let output_folder = &config.build_config.output_folder;

			panicking_create_dir_if_not_exists(&output_folder);

			if !panicking_exists(config.build_config.output_folder.join(".build-folder")) {
				event!(Level::INFO, "output folder not marked as safe");

				println!("The output folder is not confirmed to be safe to delete all files from.");
				println!(
					"This is the output folder: {}",
					panicking_canonicalize(&output_folder).display()
				);

				print!(
					"Is this folder the correct one, and is it safe to delete all files from? (y/N) "
				);
				std::io::stdout().flush().expect("failed to flush stdout");

				let mut buf = String::new();
				std::io::stdin()
					.read_line(&mut buf)
					.expect("failed to read from stdin");

				if buf.trim().to_lowercase() == "y" {
					println!("\nConfirmed, marking as safe");
					panicking_touch(config.build_config.output_folder.join(".build-folder"));
				} else {
					println!("Aborting");
					event!(Level::ERROR, "user cancelled clearing of output folder");
					return Ok(());
				}
			}

			panicking_clear_dir(&output_folder);
			panicking_create_dir_if_not_exists(output_folder.join("assets"));
			panicking_create_dir_if_not_exists(output_folder.join("posts"));

			assert_is_file("index.html");
			assert_is_file("post_template.html");

			let post_template = panicking_read_file("post_template.html");

			panicking_copy(
				"index.html",
				config.build_config.output_folder.join("index.html"),
			);
			// TODO: Templating in index.html

			let post_compile_span = span!(Level::TRACE, "compile_posts");

			{
				let _enter = post_compile_span.enter();

				assert_is_dir("posts/");
				for post_entry in panicking_read_dir("posts") {
					let post_path = post_entry.path();
					event!(Level::TRACE, "compiling post {}", post_path.display());

					let mut new_post_name = match post_path.file_stem() {
						Some(name) => match name.to_str() {
							Some(name) => name.to_string(),
							None => {
								let error_msg = format!("failed to convert {:?} to unicode", name);
								event!(Level::ERROR, "{error_msg}");
								panic!("{error_msg}");
							}
						},
						None => {
							let error_msg =
								format!("failed to obtain file prefix for {}", post_path.display());
							event!(Level::ERROR, "{error_msg}");
							panic!("{error_msg}");
						}
					};

					new_post_name.push_str(".html");
					let new_path = output_folder.join("posts").join(new_post_name);

					let markdown_contents = panicking_read_file(&post_path);
					let html = markdown::to_html(&markdown_contents);

					let new_contents = post_template.replace("{{POST_CONTENTS}}", &html);

					panicking_write_file(new_path, new_contents);
				}
			}
		}
	}

	Ok(())
}

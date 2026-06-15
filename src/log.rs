use tracing::Level;

pub enum LogLevel {
	Simple,
	Verbose
}

pub fn set_log_level(level: LogLevel) {
	match level {
		LogLevel::Simple => tracing::subscriber::set_global_default(tracing_subscriber::fmt().with_max_level(Level::INFO).finish()).expect("unable to set tracing subscriber"),
		LogLevel::Verbose => tracing::subscriber::set_global_default(tracing_subscriber::fmt().pretty().with_max_level(Level::TRACE).finish()).expect("unable to set tracing subscriber")
	}
}
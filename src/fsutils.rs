use std::{
	fs, io,
	path::{Path, PathBuf},
};

use tracing::{Level, event};

/// Panics if the operation fails
pub fn panicking_exists<T: AsRef<Path>>(path: T) -> bool {
	match fs::exists(&path) {
		Ok(res) => res,
		Err(e) => {
			let error_msg = format!("failed to check if {} exists: {e}", path.as_ref().display());
			event!(Level::ERROR, "{error_msg}");
			panic!("{error_msg}");
		}
	}
}

/// Panics if the file doesn't exist, or the operation otherwise fails
pub fn panicking_canonicalize<T: AsRef<Path>>(path: T) -> PathBuf {
	match fs::canonicalize(&path) {
		Ok(path) => path,
		Err(e) => match e.kind() {
			io::ErrorKind::NotFound => {
				let error_msg = format!(
					"expected {} to exist, but it does not",
					path.as_ref().display()
				);
				event!(Level::ERROR, "{error_msg}");
				panic!("{error_msg}");
			}
			_ => {
				let error_msg = format!("failed to evaluate path {}: {e}", path.as_ref().display());
				event!(Level::ERROR, "{error_msg}");
				panic!("{error_msg}");
			}
		},
	}
}

pub fn panicking_metadata<T: AsRef<Path>>(path: T) -> fs::Metadata {
	match fs::metadata(&path) {
		Ok(meta) => meta,
		Err(e) => {
			let error_msg = format!(
				"failed to retrieve metadata for {}: {e}",
				path.as_ref().display()
			);
			event!(Level::ERROR, "{error_msg}");
			panic!("{error_msg}");
		}
	}
}

pub struct PanickingReadDir {
	wrapped_iter: fs::ReadDir,
	path: PathBuf,
}

impl Iterator for PanickingReadDir {
	type Item = fs::DirEntry;
	fn next(&mut self) -> Option<Self::Item> {
		match self.wrapped_iter.next() {
			Some(item) => match item {
				Ok(i) => Some(i),
				Err(e) => {
					let error_msg = format!(
						"failed to access directory entry in {}: {e}",
						self.path.display()
					);
					event!(Level::ERROR, "{error_msg}");
					panic!("{error_msg}");
				}
			},
			None => None,
		}
	}
}

pub fn panicking_read_dir<T: AsRef<Path>>(path: T) -> PanickingReadDir {
	PanickingReadDir {
		wrapped_iter: match std::fs::read_dir(&path) {
			Ok(iter) => iter,
			Err(e) => {
				let error_msg =
					format!("failed to read directory {}: {e}", path.as_ref().display());
				event!(Level::ERROR, "{error_msg}");
				panic!("{error_msg}");
			}
		},
		path: path.as_ref().to_owned(),
	}
}

pub fn panicking_clear_dir<T: AsRef<Path>>(path: T) {
	assert_is_dir(&path);

	for entry in panicking_read_dir(&path) {
		let name = entry.file_name();
		if name == ".build-folder" {
			continue;
		}

		let entry_path = entry.path();
		let metadata = panicking_metadata(&entry_path);

		if metadata.is_dir() {
			match std::fs::remove_dir_all(&entry_path) {
				Ok(_) => event!(Level::DEBUG, "deleted directory {}", entry_path.display()),
				Err(e) => {
					let error_msg =
						format!("failed to delete directory {}: {e}", entry_path.display());
					event!(Level::ERROR, "{error_msg}");
					panic!("{error_msg}");
				}
			}
		} else {
			match std::fs::remove_file(&entry_path) {
				Ok(_) => event!(Level::DEBUG, "deleted file {}", entry_path.display()),
				Err(e) => {
					let error_msg = format!("failed to delete file {}: {e}", entry_path.display());
					event!(Level::ERROR, "{error_msg}");
					panic!("{error_msg}");
				}
			}
		}
	}
}

pub fn panicking_read_file<T: AsRef<Path>>(path: T) -> String {
	assert_is_file(&path);

	match fs::read_to_string(&path) {
		Ok(contents) => contents,
		Err(e) => {
			let error_msg = format!("failed to read file {}: {e}", path.as_ref().display());
			event!(Level::ERROR, "{error_msg}");
			panic!("{error_msg}");
		}
	}
}

pub fn panicking_write_file<T: AsRef<Path>, C: AsRef<[u8]>>(path: T, contents: C) {
	match fs::write(&path, contents) {
		Ok(_) => event!(Level::DEBUG, "wrote file {}", path.as_ref().display()),
		Err(e) => {
			let error_msg = format!("failed to write file {}: {e}", path.as_ref().display());
			event!(Level::ERROR, "{error_msg}");
			panic!("{error_msg}");
		}
	}
}

pub fn panicking_create_dir_if_not_exists<T: AsRef<Path>>(path: T) {
	if panicking_exists(&path) {
		event!(
			Level::TRACE,
			"{} already exists, skipping directory creation",
			path.as_ref().display()
		);
		return;
	}
	match fs::create_dir(&path) {
		Ok(_) => event!(
			Level::DEBUG,
			"created directory {}",
			path.as_ref().display()
		),
		Err(e) => {
			let error_msg = format!(
				"failed to create directory {}: {e}",
				path.as_ref().display()
			);
			event!(Level::ERROR, "{error_msg}");
			panic!("{error_msg}");
		}
	}
}

pub fn panicking_touch<T: AsRef<Path>>(path: T) {
	if panicking_exists(&path) {
		event!(
			Level::TRACE,
			"{} already exists, skipping file creation",
			path.as_ref().display()
		);
		return;
	}
	match fs::write(&path, "") {
		Ok(_) => event!(Level::DEBUG, "created file {}", path.as_ref().display()),
		Err(e) => {
			let error_msg = format!("failed to create file {}: {e}", path.as_ref().display());
			event!(Level::ERROR, "{error_msg}");
			panic!("{error_msg}");
		}
	}
}

pub fn panicking_copy<A: AsRef<Path>, B: AsRef<Path>>(from: A, to: B) {
	match fs::copy(&from, &to) {
		Ok(_) => event!(
			Level::DEBUG,
			"moved {} to {}",
			from.as_ref().display(),
			to.as_ref().display()
		),
		Err(e) => {
			let error_msg = format!(
				"failed to move {} to {}: {e}",
				from.as_ref().display(),
				to.as_ref().display()
			);
			event!(Level::ERROR, "{error_msg}");
			panic!("{error_msg}");
		}
	}
}

pub fn assert_exists<T: AsRef<Path>>(path: T) {
	let canonical_path = panicking_canonicalize(path);
	if panicking_exists(&canonical_path) {
		event!(Level::TRACE, "{} exists", canonical_path.display());
	}
}

pub fn assert_is_file<T: AsRef<Path>>(path: T) {
	let canonical_path = panicking_canonicalize(path);

	let metadata = panicking_metadata(&canonical_path);

	if metadata.is_file() {
		event!(Level::TRACE, "{} is a file", canonical_path.display());
	} else {
		let error_msg = format!(
			"expected {} to be a file, but it is not",
			canonical_path.display()
		);
		event!(Level::ERROR, "{error_msg}");
		panic!("{error_msg}");
	}
}

pub fn assert_is_dir<T: AsRef<Path>>(path: T) {
	let canonical_path = panicking_canonicalize(path);

	let metadata = panicking_metadata(&canonical_path);

	if metadata.is_dir() {
		event!(Level::TRACE, "{} is a directory", canonical_path.display());
	} else {
		let error_msg = format!(
			"expected {} to be a directory, but it is not",
			canonical_path.display()
		);
		event!(Level::ERROR, "{error_msg}");
		panic!("{error_msg}");
	}
}

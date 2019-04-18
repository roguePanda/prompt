use std::env;
use std::path::{Path, PathBuf};

pub mod mux;
pub mod net;
pub mod user;
pub mod vcs;
pub mod style;

pub use crate::style::style;

pub fn cwd() -> String {
	// TODO: path shortening?

	let home_dir = dirs::home_dir().unwrap_or(PathBuf::new());
	let current_dir = env::current_dir().unwrap_or(PathBuf::new());

	let cwd = if current_dir.starts_with(&home_dir) {
		Path::new("~").join(without_prefix(&home_dir, &current_dir))
	}
	else {
		current_dir
	};

	format!("{}", cwd.display())
}

fn without_prefix(prefix: &Path, path: &Path) -> PathBuf {
    assert!(path.starts_with(prefix));

    let mut components = path.components();
    for _ in prefix.components() {
        components.next();
    }
    return components.as_path().to_path_buf()
}
use std::path::PathBuf;

use directories::ProjectDirs;

/// Platform-specific directory for configuration and session state.
///
/// The directory is reported by `simaster doctor`; it is not created until a
/// command needs to persist state.
#[must_use]
pub fn config_dir() -> Option<PathBuf> {
    ProjectDirs::from("id", "ugm", "simaster").map(|dirs| dirs.config_dir().to_path_buf())
}

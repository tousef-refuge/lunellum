use std::path::PathBuf;

pub fn display_path(path: &PathBuf) -> String {
    path.to_string_lossy()
        .strip_prefix(r"\\?\")
        .unwrap_or(&path.to_string_lossy())
        .to_string()
}
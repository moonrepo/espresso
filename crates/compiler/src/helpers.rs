use std::path::Path;

pub fn has_extension(path: &Path, exts: &[&str]) -> bool {
    path.extension()
        .map(|ext| exts.iter().any(|e| ext.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}

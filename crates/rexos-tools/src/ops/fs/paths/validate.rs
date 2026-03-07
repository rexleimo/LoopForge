use std::path::{Path, PathBuf};

use anyhow::bail;

pub(crate) fn validate_relative_path(user_path: &str) -> anyhow::Result<PathBuf> {
    if user_path.trim().is_empty() {
        bail!("path is empty");
    }

    let path = Path::new(user_path);
    if path.is_absolute() {
        bail!("absolute paths are not allowed");
    }

    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::Normal(segment) => out.push(segment),
            std::path::Component::ParentDir => bail!("parent traversal is not allowed"),
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                bail!("invalid path")
            }
        }
    }

    if out.as_os_str().is_empty() {
        bail!("invalid path");
    }
    Ok(out)
}

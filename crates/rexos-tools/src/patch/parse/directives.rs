use anyhow::bail;

pub(super) fn add_file_path(line: &str) -> anyhow::Result<Option<String>> {
    if let Some(rest) = line.strip_prefix("*** Add File:") {
        let path = rest.trim().to_string();
        if path.is_empty() {
            bail!("empty path in '*** Add File:'");
        }
        return Ok(Some(path));
    }
    Ok(None)
}

pub(super) fn update_file_path(line: &str) -> anyhow::Result<Option<String>> {
    if let Some(rest) = line.strip_prefix("*** Update File:") {
        let path = rest.trim().to_string();
        if path.is_empty() {
            bail!("empty path in '*** Update File:'");
        }
        return Ok(Some(path));
    }
    Ok(None)
}

pub(super) fn delete_file_path(line: &str) -> anyhow::Result<Option<String>> {
    if let Some(rest) = line.strip_prefix("*** Delete File:") {
        let path = rest.trim().to_string();
        if path.is_empty() {
            bail!("empty path in '*** Delete File:'");
        }
        return Ok(Some(path));
    }
    Ok(None)
}

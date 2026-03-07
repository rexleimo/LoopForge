use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) fn file_list(&self, user_path: &str) -> anyhow::Result<String> {
        let resolved = if user_path.trim() == "." {
            self.workspace_root.clone()
        } else {
            self.resolve_workspace_path(user_path)?
        };

        let mut out = Vec::new();
        for entry in std::fs::read_dir(&resolved)
            .with_context(|| format!("list dir {}", resolved.display()))?
        {
            let entry = entry.context("read dir entry")?;
            let name = entry.file_name().to_string_lossy().to_string();
            let suffix = match entry.file_type() {
                Ok(file_type) => super::list_entry_suffix(file_type.is_dir()),
                Err(_) => super::list_entry_suffix(false),
            };
            out.push(format!("{name}{suffix}"));
        }
        out.sort();
        Ok(out.join("\n"))
    }
}

use std::path::PathBuf;

use crate::skills;

pub(super) fn run_list(workspace: PathBuf, json: bool) -> anyhow::Result<()> {
    let list = skills::list_skills(&workspace)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&list)?);
    } else if list.is_empty() {
        println!("no skills discovered");
    } else {
        for item in list {
            println!(
                "{}  v{}  source={}  entry={}",
                item.name, item.version, item.source, item.entry_path
            );
        }
    }
    Ok(())
}

pub(super) fn run_show(name: String, workspace: PathBuf, json: bool) -> anyhow::Result<()> {
    let skill = skills::find_skill(&workspace, &name)?;
    let item = serde_json::json!({
        "name": skill.name,
        "version": skill.manifest.version.to_string(),
        "source": skills::source_name(skill.source),
        "root_dir": skill.root_dir,
        "manifest_path": skill.manifest_path,
        "entry": skill.manifest.entry,
        "permissions": skill.manifest.permissions,
        "dependencies": skill
            .manifest
            .dependencies
            .iter()
            .map(|dependency| serde_json::json!({
                "name": dependency.name,
                "version_req": dependency.version_req.to_string(),
            }))
            .collect::<Vec<_>>(),
    });
    println!("{}", serde_json::to_string_pretty(&item)?);
    if !json {}
    Ok(())
}

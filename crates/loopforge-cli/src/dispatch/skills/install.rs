use std::path::PathBuf;

use anyhow::{bail, Context};
use serde::Serialize;

use super::super::json_output::{print_pretty_json, to_json_value};
use crate::cli::SkillsArchiveFormat;

const MAX_ARCHIVE_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Serialize)]
struct SkillsInstallResult {
    url: String,
    name: String,
    version: String,
    install_dir: String,
    entry_path: String,
    replaced_existing: bool,
}

pub(super) async fn run_install(
    url: String,
    workspace: PathBuf,
    format: SkillsArchiveFormat,
    force: bool,
    json: bool,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(&workspace)
        .with_context(|| format!("create workspace: {}", workspace.display()))?;
    let install_root = workspace.join(".loopforge/skills");

    let archive = download_archive(&url).await?;
    let installed = rexos_skills::installer::install_skill_archive(
        &archive,
        to_archive_format(format),
        &install_root,
        force,
    )
    .with_context(|| format!("install skill archive from `{url}`"))?;

    let output = SkillsInstallResult {
        url,
        name: installed.name,
        version: installed.version.to_string(),
        install_dir: installed.install_dir.display().to_string(),
        entry_path: installed.entry_path.display().to_string(),
        replaced_existing: installed.replaced_existing,
    };

    if json {
        print_pretty_json(&build_skills_install_json(&output)?)?;
    } else {
        println!(
            "installed skill `{}` v{} -> {}",
            output.name, output.version, output.install_dir
        );
        if output.replaced_existing {
            println!("replaced_existing: true");
        }
    }

    Ok(())
}

fn build_skills_install_json(output: &SkillsInstallResult) -> anyhow::Result<serde_json::Value> {
    to_json_value(output)
}

fn to_archive_format(format: SkillsArchiveFormat) -> rexos_skills::installer::ArchiveFormat {
    match format {
        SkillsArchiveFormat::Auto => rexos_skills::installer::ArchiveFormat::Auto,
        SkillsArchiveFormat::Zip => rexos_skills::installer::ArchiveFormat::Zip,
        SkillsArchiveFormat::Tar => rexos_skills::installer::ArchiveFormat::Tar,
        SkillsArchiveFormat::TarGz => rexos_skills::installer::ArchiveFormat::TarGz,
    }
}

async fn download_archive(url: &str) -> anyhow::Result<Vec<u8>> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!("download skill archive from `{url}`"))?;
    let status = response.status();
    if !status.is_success() {
        bail!("download failed from `{url}`: http {status}");
    }

    if let Some(length) = response.content_length() {
        if length > MAX_ARCHIVE_BYTES as u64 {
            bail!(
                "archive too large ({length} bytes), max allowed is {} bytes",
                MAX_ARCHIVE_BYTES
            );
        }
    }

    let payload = response
        .bytes()
        .await
        .with_context(|| format!("read downloaded archive body from `{url}`"))?;
    if payload.is_empty() {
        bail!("downloaded archive is empty");
    }
    if payload.len() > MAX_ARCHIVE_BYTES {
        bail!(
            "archive too large ({} bytes), max allowed is {} bytes",
            payload.len(),
            MAX_ARCHIVE_BYTES
        );
    }

    Ok(payload.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_skills_install_json_keeps_expected_shape() {
        let out = SkillsInstallResult {
            url: "https://example.com/hello.zip".to_string(),
            name: "hello".to_string(),
            version: "0.1.0".to_string(),
            install_dir: "/tmp/workspace/.loopforge/skills/hello".to_string(),
            entry_path: "/tmp/workspace/.loopforge/skills/hello/SKILL.md".to_string(),
            replaced_existing: false,
        };
        let value = build_skills_install_json(&out).unwrap();
        assert_eq!(
            value,
            json!({
                "url": "https://example.com/hello.zip",
                "name": "hello",
                "version": "0.1.0",
                "install_dir": "/tmp/workspace/.loopforge/skills/hello",
                "entry_path": "/tmp/workspace/.loopforge/skills/hello/SKILL.md",
                "replaced_existing": false
            })
        );
    }

    #[test]
    fn to_archive_format_maps_all_variants() {
        assert_eq!(
            to_archive_format(SkillsArchiveFormat::Auto),
            rexos_skills::installer::ArchiveFormat::Auto
        );
        assert_eq!(
            to_archive_format(SkillsArchiveFormat::Zip),
            rexos_skills::installer::ArchiveFormat::Zip
        );
        assert_eq!(
            to_archive_format(SkillsArchiveFormat::Tar),
            rexos_skills::installer::ArchiveFormat::Tar
        );
        assert_eq!(
            to_archive_format(SkillsArchiveFormat::TarGz),
            rexos_skills::installer::ArchiveFormat::TarGz
        );
    }
}

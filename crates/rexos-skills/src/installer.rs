use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};

use crate::manifest::parse_manifest;

const SKILL_MANIFEST_FILE: &str = "skill.toml";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    Auto,
    Zip,
    Tar,
    TarGz,
}

#[derive(Debug, Clone)]
pub struct InstalledSkill {
    pub name: String,
    pub version: semver::Version,
    pub install_dir: PathBuf,
    pub entry_path: PathBuf,
    pub replaced_existing: bool,
}

pub fn install_skill_archive(
    archive_bytes: &[u8],
    archive_format: ArchiveFormat,
    install_root: &Path,
    force: bool,
) -> anyhow::Result<InstalledSkill> {
    std::fs::create_dir_all(install_root)
        .with_context(|| format!("create install root: {}", install_root.display()))?;
    let canonical_install_root = install_root
        .canonicalize()
        .with_context(|| format!("canonicalize install root: {}", install_root.display()))?;

    let staging = tempfile::tempdir().context("create staging directory")?;
    let staging_extract_root = staging.path().join("extract");
    std::fs::create_dir_all(&staging_extract_root).context("create extraction staging root")?;
    extract_archive_bytes(archive_bytes, archive_format, &staging_extract_root)?;

    let extracted_skill_root = detect_extracted_skill_root(&staging_extract_root)?;
    let manifest_path = extracted_skill_root.join(SKILL_MANIFEST_FILE);
    let manifest_raw = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("read manifest: {}", manifest_path.display()))?;
    let manifest = parse_manifest(&manifest_raw)
        .with_context(|| format!("parse manifest: {}", manifest_path.display()))?;
    validate_skill_name_segment(&manifest.name)?;

    let extracted_entry = extracted_skill_root.join(manifest.entry.trim());
    if !extracted_entry.is_file() {
        bail!(
            "skill entry file missing after extraction: {}",
            extracted_entry.display()
        );
    }

    let destination_dir = canonical_install_root.join(&manifest.name);
    let mut replaced_existing = false;
    if destination_dir.exists() {
        if !force {
            bail!(
                "skill `{}` already exists at {}; rerun with --force to replace it",
                manifest.name,
                destination_dir.display()
            );
        }
        std::fs::remove_dir_all(&destination_dir).with_context(|| {
            format!(
                "remove existing installed skill: {}",
                destination_dir.display()
            )
        })?;
        replaced_existing = true;
    }

    move_dir_with_fallback(&extracted_skill_root, &destination_dir)?;
    let entry_path = destination_dir.join(manifest.entry.trim());

    Ok(InstalledSkill {
        name: manifest.name,
        version: manifest.version,
        install_dir: destination_dir,
        entry_path,
        replaced_existing,
    })
}

pub fn extract_archive_bytes(
    archive_bytes: &[u8],
    archive_format: ArchiveFormat,
    destination_root: &Path,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(destination_root)
        .with_context(|| format!("create extraction root: {}", destination_root.display()))?;
    let canonical_root = destination_root.canonicalize().with_context(|| {
        format!(
            "canonicalize extraction root: {}",
            destination_root.display()
        )
    })?;

    let resolved_format = match archive_format {
        ArchiveFormat::Auto => detect_archive_format(archive_bytes)?,
        explicit => explicit,
    };

    match resolved_format {
        ArchiveFormat::Auto => unreachable!("auto must be resolved before extraction"),
        ArchiveFormat::Zip => extract_zip_archive(archive_bytes, &canonical_root),
        ArchiveFormat::Tar => extract_tar_archive(archive_bytes, &canonical_root),
        ArchiveFormat::TarGz => extract_targz_archive(archive_bytes, &canonical_root),
    }
}

fn extract_zip_archive(archive_bytes: &[u8], canonical_root: &Path) -> anyhow::Result<()> {
    let cursor = Cursor::new(archive_bytes);
    let mut archive = zip::ZipArchive::new(cursor).context("parse zip archive")?;

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .with_context(|| format!("read zip entry at index {index}"))?;
        let entry_name = entry.name().to_string();
        if is_zip_symlink(&entry) {
            bail!("archive symlink entries are not allowed: `{entry_name}`");
        }
        let relative_path = normalize_archive_relative_path(&entry_name)
            .with_context(|| format!("zip entry `{entry_name}`"))?;
        let destination = canonical_root.join(&relative_path);

        if entry.is_dir() {
            create_safe_dir(&destination, canonical_root)?;
            continue;
        }

        if let Some(parent) = destination.parent() {
            create_safe_dir(parent, canonical_root)?;
        } else {
            bail!("zip entry has invalid destination path: `{entry_name}`");
        }
        ensure_existing_path_within_root(&destination, canonical_root)?;

        let mut output = std::fs::File::create(&destination)
            .with_context(|| format!("create extracted file: {}", destination.display()))?;
        std::io::copy(&mut entry, &mut output)
            .with_context(|| format!("write extracted file: {}", destination.display()))?;
    }

    Ok(())
}

fn extract_tar_archive(archive_bytes: &[u8], canonical_root: &Path) -> anyhow::Result<()> {
    let reader = Cursor::new(archive_bytes);
    extract_tar_reader(reader, canonical_root)
}

fn extract_targz_archive(archive_bytes: &[u8], canonical_root: &Path) -> anyhow::Result<()> {
    let decoder = flate2::read::GzDecoder::new(Cursor::new(archive_bytes));
    extract_tar_reader(decoder, canonical_root)
}

fn extract_tar_reader<R: Read>(reader: R, canonical_root: &Path) -> anyhow::Result<()> {
    let mut archive = tar::Archive::new(reader);
    for entry_result in archive.entries().context("read tar archive entries")? {
        let mut entry = entry_result.context("read tar entry")?;
        let path = entry.path().context("read tar entry path")?;
        let entry_name = path.to_string_lossy().into_owned();
        let relative_path = normalize_archive_relative_path(&entry_name)
            .with_context(|| format!("tar entry `{entry_name}`"))?;
        let destination = canonical_root.join(&relative_path);
        let entry_type = entry.header().entry_type();

        if entry_type.is_symlink() || entry_type.is_hard_link() {
            bail!("archive symlink entries are not allowed: `{entry_name}`");
        }
        if entry_type.is_dir() {
            create_safe_dir(&destination, canonical_root)?;
            continue;
        }
        if entry_type.is_file() || entry_type.is_contiguous() {
            if let Some(parent) = destination.parent() {
                create_safe_dir(parent, canonical_root)?;
            } else {
                bail!("tar entry has invalid destination path: `{entry_name}`");
            }
            ensure_existing_path_within_root(&destination, canonical_root)?;
            let mut output = std::fs::File::create(&destination)
                .with_context(|| format!("create extracted file: {}", destination.display()))?;
            std::io::copy(&mut entry, &mut output)
                .with_context(|| format!("write extracted file: {}", destination.display()))?;
            continue;
        }
        if entry_type.is_pax_global_extensions()
            || entry_type.is_pax_local_extensions()
            || entry_type.is_gnu_longname()
            || entry_type.is_gnu_longlink()
        {
            continue;
        }

        bail!("unsupported tar entry type for `{entry_name}`");
    }

    Ok(())
}

fn is_zip_symlink(entry: &zip::read::ZipFile<'_>) -> bool {
    if let Some(mode) = entry.unix_mode() {
        return mode & 0o170000 == 0o120000;
    }
    false
}

fn detect_archive_format(archive_bytes: &[u8]) -> anyhow::Result<ArchiveFormat> {
    let is_zip = archive_bytes.starts_with(b"PK\x03\x04")
        || archive_bytes.starts_with(b"PK\x05\x06")
        || archive_bytes.starts_with(b"PK\x07\x08");
    if is_zip {
        return Ok(ArchiveFormat::Zip);
    }
    if archive_bytes.starts_with(&[0x1f, 0x8b]) {
        return Ok(ArchiveFormat::TarGz);
    }

    // POSIX ustar indicator at offset 257.
    if archive_bytes.len() >= 262 && &archive_bytes[257..262] == b"ustar" {
        return Ok(ArchiveFormat::Tar);
    }

    bail!("unsupported archive format (expected zip, tar, or tar.gz)")
}

fn normalize_archive_relative_path(raw: &str) -> anyhow::Result<PathBuf> {
    let candidate = raw.trim();
    if candidate.is_empty() {
        bail!("archive entry path is empty");
    }
    if candidate.starts_with('/') || candidate.starts_with('\\') {
        bail!("archive entry uses an absolute path");
    }
    if candidate.len() >= 2
        && candidate.as_bytes()[1] == b':'
        && candidate.as_bytes()[0].is_ascii_alphabetic()
    {
        bail!("archive entry uses an absolute path");
    }

    let mut normalized = PathBuf::new();
    for segment in candidate.split(['/', '\\']) {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." {
            bail!("archive entry contains parent traversal");
        }
        normalized.push(segment);
    }

    if normalized.as_os_str().is_empty() {
        bail!("archive entry path is empty after normalization");
    }

    Ok(normalized)
}

fn create_safe_dir(path: &Path, canonical_root: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(path)
        .with_context(|| format!("create directory: {}", path.display()))?;
    let canonical = path
        .canonicalize()
        .with_context(|| format!("canonicalize directory: {}", path.display()))?;
    ensure_within_root(&canonical, canonical_root, path)
}

fn ensure_existing_path_within_root(path: &Path, canonical_root: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        return Ok(());
    }
    let canonical = path
        .canonicalize()
        .with_context(|| format!("canonicalize path: {}", path.display()))?;
    ensure_within_root(&canonical, canonical_root, path)
}

fn ensure_within_root(
    canonical_path: &Path,
    canonical_root: &Path,
    display_path: &Path,
) -> anyhow::Result<()> {
    if canonical_path.starts_with(canonical_root) {
        return Ok(());
    }
    bail!(
        "archive entry escaped extraction root: {}",
        display_path.display()
    );
}

fn detect_extracted_skill_root(extraction_root: &Path) -> anyhow::Result<PathBuf> {
    if extraction_root.join(SKILL_MANIFEST_FILE).is_file() {
        return Ok(extraction_root.to_path_buf());
    }

    let mut candidates = Vec::new();
    for entry in std::fs::read_dir(extraction_root)
        .with_context(|| format!("read extraction root: {}", extraction_root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let candidate = entry.path();
        if candidate.join(SKILL_MANIFEST_FILE).is_file() {
            candidates.push(candidate);
        }
    }

    match candidates.len() {
        1 => Ok(candidates.remove(0)),
        0 => bail!("archive is missing `skill.toml` at root or one top-level directory"),
        _ => bail!("archive contains multiple top-level skills; expected exactly one"),
    }
}

fn validate_skill_name_segment(name: &str) -> anyhow::Result<()> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        bail!("skill name cannot be empty");
    }
    if trimmed == "." || trimmed == ".." {
        bail!("skill name cannot be `.` or `..`");
    }
    if trimmed.contains('/') || trimmed.contains('\\') {
        bail!("skill name cannot contain path separators");
    }
    if trimmed.chars().any(char::is_control) {
        bail!("skill name cannot contain control characters");
    }
    Ok(())
}

fn move_dir_with_fallback(src: &Path, dst: &Path) -> anyhow::Result<()> {
    if dst.exists() {
        bail!("destination already exists: {}", dst.display());
    }
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create destination parent: {}", parent.display()))?;
    }

    if std::fs::rename(src, dst).is_ok() {
        return Ok(());
    }

    copy_dir_recursive(src, dst)?;
    std::fs::remove_dir_all(src).with_context(|| {
        format!(
            "remove source directory after fallback copy: {}",
            src.display()
        )
    })
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(dst).with_context(|| format!("create directory: {}", dst.display()))?;
    for entry in
        std::fs::read_dir(src).with_context(|| format!("read directory: {}", src.display()))?
    {
        let entry = entry?;
        let entry_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if entry_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
            continue;
        }
        if entry_type.is_symlink() {
            bail!(
                "refusing to copy symlink from extracted archive: {}",
                src_path.display()
            );
        }
        if entry_type.is_file() {
            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("create directory: {}", parent.display()))?;
            }
            std::fs::copy(&src_path, &dst_path).with_context(|| {
                format!(
                    "copy extracted file from {} to {}",
                    src_path.display(),
                    dst_path.display()
                )
            })?;
            continue;
        }
        bail!("unsupported extracted entry type: {}", src_path.display());
    }
    Ok(())
}

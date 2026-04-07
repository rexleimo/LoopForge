use std::io::{Cursor, Write};

use rexos_skills::installer::{extract_archive_bytes, install_skill_archive, ArchiveFormat};

fn build_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = zip::ZipWriter::new(&mut cursor);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        for (path, data) in entries {
            if path.ends_with('/') {
                writer.add_directory(*path, options).unwrap();
            } else {
                writer.start_file(*path, options).unwrap();
                writer.write_all(data).unwrap();
            }
        }
        writer.finish().unwrap();
    }
    cursor.into_inner()
}

fn build_raw_tar_single_file(path: &str, data: &[u8]) -> Vec<u8> {
    fn write_octal(dst: &mut [u8], value: u64) {
        let width = dst.len();
        let mut encoded = format!("{value:o}");
        if encoded.len() + 1 > width {
            encoded = "0".repeat(width.saturating_sub(1));
        }
        let start = width.saturating_sub(encoded.len() + 1);
        for b in dst.iter_mut() {
            *b = b'0';
        }
        dst[start..start + encoded.len()].copy_from_slice(encoded.as_bytes());
        dst[width - 1] = 0;
    }

    let mut header = [0u8; 512];
    let path_bytes = path.as_bytes();
    assert!(path_bytes.len() <= 100);
    header[..path_bytes.len()].copy_from_slice(path_bytes);
    write_octal(&mut header[100..108], 0o644);
    write_octal(&mut header[108..116], 0);
    write_octal(&mut header[116..124], 0);
    write_octal(&mut header[124..136], data.len() as u64);
    write_octal(&mut header[136..148], 0);
    header[148..156].fill(b' ');
    header[156] = b'0';
    header[257..263].copy_from_slice(b"ustar\0");
    header[263..265].copy_from_slice(b"00");

    let checksum: u32 = header.iter().map(|b| *b as u32).sum();
    let checksum_text = format!("{checksum:06o}\0 ");
    header[148..156].copy_from_slice(checksum_text.as_bytes());

    let mut out = Vec::new();
    out.extend_from_slice(&header);
    out.extend_from_slice(data);
    let remainder = data.len() % 512;
    if remainder != 0 {
        out.resize(out.len() + (512 - remainder), 0);
    }
    out.extend_from_slice(&[0u8; 1024]);
    out
}

fn build_tar_with_symlink(path: &str, link_name: &str) -> Vec<u8> {
    let mut out = Vec::new();
    {
        let mut builder = tar::Builder::new(&mut out);
        let mut header = tar::Header::new_gnu();
        header.set_path(path).unwrap();
        header.set_link_name(link_name).unwrap();
        header.set_entry_type(tar::EntryType::Symlink);
        header.set_mode(0o755);
        header.set_size(0);
        header.set_cksum();
        builder.append(&header, std::io::empty()).unwrap();
        builder.finish().unwrap();
    }
    out
}

#[test]
fn install_skill_archive_extracts_and_installs() {
    let archive = build_zip(&[
        (
            "hello-skill/skill.toml",
            br#"name = "hello-skill"
version = "0.1.0"
entry = "SKILL.md"
"#,
        ),
        ("hello-skill/SKILL.md", b"# hello"),
    ]);
    let tmp = tempfile::tempdir().unwrap();
    let install_root = tmp.path().join("skills");

    let installed =
        install_skill_archive(&archive, ArchiveFormat::Auto, &install_root, false).unwrap();

    assert_eq!(installed.name, "hello-skill");
    assert_eq!(installed.version.to_string(), "0.1.0");
    assert!(!installed.replaced_existing);
    assert!(installed.install_dir.join("skill.toml").is_file());
    assert!(installed.entry_path.is_file());
}

#[test]
fn install_skill_archive_rejects_existing_without_force() {
    let v1 = build_zip(&[
        (
            "dup-skill/skill.toml",
            br#"name = "dup-skill"
version = "0.1.0"
entry = "SKILL.md"
"#,
        ),
        ("dup-skill/SKILL.md", b"v1"),
    ]);
    let v2 = build_zip(&[
        (
            "dup-skill/skill.toml",
            br#"name = "dup-skill"
version = "0.2.0"
entry = "SKILL.md"
"#,
        ),
        ("dup-skill/SKILL.md", b"v2"),
    ]);
    let tmp = tempfile::tempdir().unwrap();
    let install_root = tmp.path().join("skills");

    install_skill_archive(&v1, ArchiveFormat::Auto, &install_root, false).unwrap();
    let err = install_skill_archive(&v2, ArchiveFormat::Auto, &install_root, false).unwrap_err();
    assert!(err.to_string().contains("already exists"));

    let replaced = install_skill_archive(&v2, ArchiveFormat::Auto, &install_root, true).unwrap();
    assert!(replaced.replaced_existing);
    assert_eq!(replaced.version.to_string(), "0.2.0");
}

#[test]
fn extract_archive_rejects_zip_parent_traversal() {
    let archive = build_zip(&[("../escape.txt", b"pwned")]);
    let tmp = tempfile::tempdir().unwrap();
    let err = extract_archive_bytes(&archive, ArchiveFormat::Zip, tmp.path()).unwrap_err();
    let message = format!("{err:#}").to_lowercase();
    assert!(
        message.contains("parent traversal") || message.contains("invalid file path"),
        "unexpected error: {message}"
    );
}

#[test]
fn extract_archive_rejects_zip_absolute_windows_prefix() {
    let archive = build_zip(&[("C:\\escape.txt", b"pwned")]);
    let tmp = tempfile::tempdir().unwrap();
    let err = extract_archive_bytes(&archive, ArchiveFormat::Zip, tmp.path()).unwrap_err();
    let message = format!("{err:#}").to_lowercase();
    assert!(
        message.contains("absolute path") || message.contains("invalid file path"),
        "unexpected error: {message}"
    );
}

#[test]
fn extract_archive_rejects_tar_parent_traversal() {
    let archive = build_raw_tar_single_file("../escape.txt", b"pwned");
    let tmp = tempfile::tempdir().unwrap();
    let err = extract_archive_bytes(&archive, ArchiveFormat::Tar, tmp.path()).unwrap_err();
    let message = format!("{err:#}").to_lowercase();
    assert!(
        message.contains("parent traversal"),
        "unexpected error: {message}"
    );
}

#[test]
fn extract_archive_rejects_tar_symlink_entries() {
    let archive = build_tar_with_symlink("sym", "../outside");
    let tmp = tempfile::tempdir().unwrap();
    let err = extract_archive_bytes(&archive, ArchiveFormat::Tar, tmp.path()).unwrap_err();
    assert!(err.to_string().contains("symlink"));
}

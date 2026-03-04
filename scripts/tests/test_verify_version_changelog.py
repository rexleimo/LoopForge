import importlib.util
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "verify_version_changelog.py"
    spec = importlib.util.spec_from_file_location("verify_version_changelog", module_path)
    module = importlib.util.module_from_spec(spec)
    assert spec is not None and spec.loader is not None
    spec.loader.exec_module(module)
    return module


mod = load_module()


class VerifyVersionChangelogTests(unittest.TestCase):
    def test_extract_workspace_version(self):
        cargo = """
[workspace]
members = []

[workspace.package]
version = "0.2.3"
edition = "2021"
"""
        self.assertEqual(mod.extract_workspace_version(cargo), "0.2.3")

    def test_pass_when_version_not_changed(self):
        ok, message = mod.evaluate_rule(
            base_version="0.1.0",
            head_version="0.1.0",
            changed_files={"Cargo.toml"},
            changelog_text="# Changelog\n\n## [Unreleased]\n",
        )
        self.assertTrue(ok)
        self.assertIn("version unchanged", message)

    def test_fail_when_version_changed_without_changelog_file_change(self):
        ok, message = mod.evaluate_rule(
            base_version="0.1.0",
            head_version="0.1.1",
            changed_files={"Cargo.toml", "crates/rexos-cli/src/main.rs"},
            changelog_text="# Changelog\n\n## [Unreleased]\n",
        )
        self.assertFalse(ok)
        self.assertIn("CHANGELOG.md must be updated", message)

    def test_fail_when_changelog_missing_target_version_heading(self):
        ok, message = mod.evaluate_rule(
            base_version="0.1.0",
            head_version="0.1.1",
            changed_files={"Cargo.toml", "CHANGELOG.md"},
            changelog_text="# Changelog\n\n## [Unreleased]\n\n### Changed\n- something\n",
        )
        self.assertFalse(ok)
        self.assertIn("missing section", message)

    def test_pass_when_version_changed_and_changelog_has_target_section(self):
        ok, message = mod.evaluate_rule(
            base_version="0.1.0",
            head_version="0.1.1",
            changed_files={"Cargo.toml", "CHANGELOG.md"},
            changelog_text="# Changelog\n\n## [0.1.1] - 2026-03-04\n\n### Added\n- item\n",
        )
        self.assertTrue(ok)
        self.assertIn("rule satisfied", message)

    def test_fail_when_tracked_docs_with_hardcoded_version_not_updated(self):
        ok, message = mod.evaluate_rule(
            base_version="0.1.0",
            head_version="0.1.1",
            changed_files={"Cargo.toml", "CHANGELOG.md"},
            changelog_text="# Changelog\n\n## [0.1.1] - 2026-03-04\n",
            tracked_docs={
                "README.md": ("Install v0.1.0 now", "Install v0.1.0 now"),
            },
        )
        self.assertFalse(ok)
        self.assertIn("README.md", message)
        self.assertIn("must be updated", message)

    def test_fail_when_tracked_docs_changed_but_missing_new_version(self):
        ok, message = mod.evaluate_rule(
            base_version="0.1.0",
            head_version="0.1.1",
            changed_files={"Cargo.toml", "CHANGELOG.md", "README.md"},
            changelog_text="# Changelog\n\n## [0.1.1] - 2026-03-04\n",
            tracked_docs={
                "README.md": ("Install v0.1.0 now", "Install latest release now"),
            },
        )
        self.assertFalse(ok)
        self.assertIn("missing new version", message)

    def test_pass_when_tracked_docs_include_new_version(self):
        ok, message = mod.evaluate_rule(
            base_version="0.1.0",
            head_version="0.1.1",
            changed_files={"Cargo.toml", "CHANGELOG.md", "README.md"},
            changelog_text="# Changelog\n\n## [0.1.1] - 2026-03-04\n",
            tracked_docs={
                "README.md": ("Install v0.1.0 now", "Install v0.1.1 now"),
            },
        )
        self.assertTrue(ok)


if __name__ == "__main__":
    unittest.main()

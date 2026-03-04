import importlib.util
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "verify_release_consistency.py"
    spec = importlib.util.spec_from_file_location("verify_release_consistency", module_path)
    module = importlib.util.module_from_spec(spec)
    assert spec is not None and spec.loader is not None
    spec.loader.exec_module(module)
    return module


mod = load_module()


class VerifyReleaseConsistencyTests(unittest.TestCase):
    def test_extract_workspace_version(self):
        cargo = """
[workspace]
members = []

[workspace.package]
version = "1.2.3"
edition = "2021"
"""
        self.assertEqual(mod.extract_workspace_version(cargo), "1.2.3")

    def test_fail_when_tag_is_not_semver_with_v_prefix(self):
        ok, message = mod.evaluate_release_consistency(
            tag="release-1.2.3",
            cargo_version="1.2.3",
            changelog_text="## [1.2.3] - 2026-03-04\n",
        )
        self.assertFalse(ok)
        self.assertIn("must match vX.Y.Z", message)

    def test_fail_when_tag_and_cargo_version_mismatch(self):
        ok, message = mod.evaluate_release_consistency(
            tag="v1.2.4",
            cargo_version="1.2.3",
            changelog_text="## [1.2.4] - 2026-03-04\n",
        )
        self.assertFalse(ok)
        self.assertIn("does not match", message)

    def test_fail_when_changelog_missing_target_section(self):
        ok, message = mod.evaluate_release_consistency(
            tag="v1.2.3",
            cargo_version="1.2.3",
            changelog_text="## [Unreleased]\n",
        )
        self.assertFalse(ok)
        self.assertIn("CHANGELOG.md is missing section", message)

    def test_pass_when_tag_cargo_and_changelog_are_consistent(self):
        ok, message = mod.evaluate_release_consistency(
            tag="v1.2.3",
            cargo_version="1.2.3",
            changelog_text="# Changelog\n\n## [1.2.3] - 2026-03-04\n\n### Added\n- item\n",
        )
        self.assertTrue(ok)
        self.assertIn("consistent", message)


if __name__ == "__main__":
    unittest.main()

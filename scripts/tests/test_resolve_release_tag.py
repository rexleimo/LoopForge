import importlib.util
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "resolve_release_tag.py"
    spec = importlib.util.spec_from_file_location("resolve_release_tag", module_path)
    module = importlib.util.module_from_spec(spec)
    assert spec is not None and spec.loader is not None
    spec.loader.exec_module(module)
    return module


mod = load_module()


class ResolveReleaseTagTests(unittest.TestCase):
    def test_creates_tag_when_workspace_version_is_ahead_of_existing_tags(self):
        should_tag, tag, reason = mod.decide_release_tag(
            workspace_version="1.3.0",
            changelog_text="# Changelog\n\n## [1.3.0] - 2026-03-07\n",
            existing_tags=["v1.2.0"],
        )
        self.assertTrue(should_tag)
        self.assertEqual(tag, "v1.3.0")
        self.assertIn("create release tag", reason)

    def test_skips_when_exact_tag_already_exists(self):
        should_tag, tag, reason = mod.decide_release_tag(
            workspace_version="1.3.0",
            changelog_text="# Changelog\n\n## [1.3.0] - 2026-03-07\n",
            existing_tags=["v1.2.0", "v1.3.0"],
        )
        self.assertFalse(should_tag)
        self.assertIsNone(tag)
        self.assertIn("already exists", reason)

    def test_fails_when_workspace_version_is_behind_latest_tag(self):
        with self.assertRaises(ValueError) as ctx:
            mod.decide_release_tag(
                workspace_version="1.2.0",
                changelog_text="# Changelog\n\n## [1.2.0] - 2026-03-07\n",
                existing_tags=["v1.3.0"],
            )
        self.assertIn("behind latest release tag", str(ctx.exception))

    def test_fails_when_changelog_section_is_missing_for_new_version(self):
        with self.assertRaises(ValueError) as ctx:
            mod.decide_release_tag(
                workspace_version="1.3.0",
                changelog_text="# Changelog\n\n## [Unreleased]\n",
                existing_tags=["v1.2.0"],
            )
        self.assertIn("CHANGELOG.md is missing section", str(ctx.exception))

    def test_latest_semver_tag_ignores_non_semver_entries(self):
        self.assertEqual(
            mod.latest_semver_tag(["preview", "v1.2.0", "v1.10.0", "v1.3.4"]),
            "v1.10.0",
        )


if __name__ == "__main__":
    unittest.main()

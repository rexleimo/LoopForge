import importlib.util
import tempfile
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "x_post_lint.py"
    spec = importlib.util.spec_from_file_location("x_post_lint", module_path)
    module = importlib.util.module_from_spec(spec)
    assert spec is not None and spec.loader is not None
    spec.loader.exec_module(module)
    return module


mod = load_module()


class XPostLintTests(unittest.TestCase):
    def test_split_blocks_uses_separator(self):
        text = "first post\n---\nsecond post\nline2\n---\nthird post\n"
        blocks = mod.split_blocks(text)
        self.assertEqual(blocks, ["first post", "second post\nline2", "third post"])

    def test_weighted_length_counts_urls_and_wide_chars(self):
        text = "LoopForge 上线 https://example.com/path"
        length = mod.weighted_length(text, url_weight=23)
        # "LoopForge " (10) + "上线" (4, wide chars) + " " (1) + URL (23)
        self.assertEqual(length, 38)

    def test_evaluate_posts_marks_over_limit(self):
        posts = [("post-1", "a" * 281), ("post-2", "b" * 250)]
        rows = mod.evaluate_posts(posts=posts, limit=280, warn_at=260, url_weight=23)
        self.assertEqual(rows[0]["status"], "OVER")
        self.assertEqual(rows[1]["status"], "OK")

    def test_main_returns_nonzero_when_any_post_is_over_limit(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "posts.txt"
            path.write_text("short\n---\n" + ("x" * 281) + "\n", encoding="utf-8")
            exit_code = mod.main(["--file", str(path), "--limit", "280", "--warn-at", "260"])
            self.assertEqual(exit_code, 1)


if __name__ == "__main__":
    unittest.main()

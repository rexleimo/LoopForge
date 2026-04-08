import importlib.util
import tempfile
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "check_i18n_parity.py"
    spec = importlib.util.spec_from_file_location("check_i18n_parity", module_path)
    module = importlib.util.module_from_spec(spec)
    assert spec is not None and spec.loader is not None
    spec.loader.exec_module(module)
    return module


mod = load_module()


class CheckI18nParityTests(unittest.TestCase):
    def test_collect_missing_translations_reports_untranslated_pages(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "docs-site"
            (root / "how-to").mkdir(parents=True, exist_ok=True)
            (root / "zh-CN/how-to").mkdir(parents=True, exist_ok=True)
            (root / "index.md").write_text("# en\n", encoding="utf-8")
            (root / "how-to/install.md").write_text("# en install\n", encoding="utf-8")
            (root / "zh-CN/index.md").write_text("# zh\n", encoding="utf-8")

            missing = mod.collect_missing_translations(root, "zh-CN")
            self.assertEqual([p.as_posix() for p in missing], ["how-to/install.md"])

    def test_collect_missing_translations_returns_empty_when_fully_mirrored(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "docs-site"
            (root / "how-to").mkdir(parents=True, exist_ok=True)
            (root / "zh-CN/how-to").mkdir(parents=True, exist_ok=True)
            (root / "index.md").write_text("# en\n", encoding="utf-8")
            (root / "how-to/install.md").write_text("# en install\n", encoding="utf-8")
            (root / "zh-CN/index.md").write_text("# zh\n", encoding="utf-8")
            (root / "zh-CN/how-to/install.md").write_text("# zh install\n", encoding="utf-8")

            missing = mod.collect_missing_translations(root, "zh-CN")
            self.assertEqual(missing, [])


if __name__ == "__main__":
    unittest.main()

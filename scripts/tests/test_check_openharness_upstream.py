import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "check_openharness_upstream.py"
    spec = importlib.util.spec_from_file_location("check_openharness_upstream", module_path)
    module = importlib.util.module_from_spec(spec)
    assert spec is not None and spec.loader is not None
    spec.loader.exec_module(module)
    return module


mod = load_module()


class CheckOpenHarnessUpstreamTests(unittest.TestCase):
    def test_parse_ls_remote_head_extracts_sha(self):
        output = "f39488c97ae9f82716c3f9d192fc4d8add50a3e7\tHEAD\n"
        self.assertEqual(
            mod.parse_ls_remote_head(output),
            "f39488c97ae9f82716c3f9d192fc4d8add50a3e7",
        )

    def test_parse_ls_remote_head_rejects_invalid_output(self):
        with self.assertRaises(ValueError):
            mod.parse_ls_remote_head("not-a-sha HEAD\n")

    def test_build_report_marks_changed_and_compare_url(self):
        report = mod.build_report(
            repo_url="https://github.com/MaxGfeller/open-harness.git",
            previous_head="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            current_head="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            checked_at="2026-04-07T00:00:00Z",
        )
        self.assertTrue(report["changed"])
        self.assertIn("compare_url", report)
        self.assertIn("aaaa", report["compare_url"])
        self.assertIn("bbbb", report["compare_url"])

    def test_main_updates_state_and_emits_outputs(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            state_path = tmp_path / "open-harness-upstream.json"
            json_out = tmp_path / "report.json"
            md_out = tmp_path / "report.md"
            state_path.write_text(
                json.dumps(
                    {
                        "tracked_repo": "https://github.com/MaxGfeller/open-harness.git",
                        "last_known_head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "last_checked_at": "2026-04-06T00:00:00Z",
                    }
                )
                + "\n",
                encoding="utf-8",
            )

            original_fetch = mod.fetch_remote_head
            try:
                mod.fetch_remote_head = lambda *_: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                rc = mod.main(
                    [
                        "--state-file",
                        str(state_path),
                        "--out-json",
                        str(json_out),
                        "--out-markdown",
                        str(md_out),
                        "--update-state",
                        "--checked-at",
                        "2026-04-07T00:00:00Z",
                    ]
                )
            finally:
                mod.fetch_remote_head = original_fetch

            self.assertEqual(rc, 0)
            persisted = json.loads(state_path.read_text(encoding="utf-8"))
            self.assertEqual(
                persisted["last_known_head"],
                "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            )
            report = json.loads(json_out.read_text(encoding="utf-8"))
            self.assertTrue(report["changed"])
            self.assertIn("compare_url", report)
            md = md_out.read_text(encoding="utf-8")
            self.assertIn("Open-Harness Upstream Watch Report", md)

    def test_main_fail_on_change_returns_nonzero(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            state_path = tmp_path / "open-harness-upstream.json"
            state_path.write_text(
                json.dumps(
                    {
                        "tracked_repo": "https://github.com/MaxGfeller/open-harness.git",
                        "last_known_head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "last_checked_at": "2026-04-06T00:00:00Z",
                    }
                )
                + "\n",
                encoding="utf-8",
            )

            original_fetch = mod.fetch_remote_head
            try:
                mod.fetch_remote_head = lambda *_: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                rc = mod.main(
                    [
                        "--state-file",
                        str(state_path),
                        "--fail-on-change",
                        "--checked-at",
                        "2026-04-07T00:00:00Z",
                    ]
                )
            finally:
                mod.fetch_remote_head = original_fetch

            self.assertEqual(rc, 1)


if __name__ == "__main__":
    unittest.main()

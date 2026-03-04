import importlib.util
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "provider_health_report.py"
    spec = importlib.util.spec_from_file_location("provider_health_report", module_path)
    module = importlib.util.module_from_spec(spec)
    assert spec is not None and spec.loader is not None
    spec.loader.exec_module(module)
    return module


mod = load_module()


class ProviderHealthReportTests(unittest.TestCase):
    def test_build_smoke_cases_always_includes_ollama(self):
        env = {
            "REXOS_OLLAMA_MODEL": "qwen3:4b",
        }
        cases = mod.build_smoke_cases(env)
        ids = [c["id"] for c in cases]
        self.assertIn("ollama_smoke", ids)

    def test_build_smoke_cases_adds_provider_when_key_present(self):
        env = {
            "REXOS_OLLAMA_MODEL": "qwen3:4b",
            "ZHIPUAI_API_KEY": "id.secret",
            "REXOS_GLM_MODEL": "glm-4",
            "MINIMAX_API_KEY": "k",
            "REXOS_MINIMAX_MODEL": "MiniMax-M2.5",
            "NVIDIA_API_KEY": "k",
            "REXOS_NVIDIA_MODEL": "meta/llama-3.2-3b-instruct",
        }
        cases = mod.build_smoke_cases(env)
        ids = [c["id"] for c in cases]
        self.assertIn("zhipu_smoke", ids)
        self.assertIn("minimax_smoke", ids)
        self.assertIn("nvidia_smoke", ids)

    def test_markdown_report_contains_summary_table(self):
        report = {
            "generated_at": "2026-03-05T00:00:00Z",
            "results": [
                {"id": "ollama_smoke", "status": "pass", "duration_sec": 7.1},
                {"id": "zhipu_smoke", "status": "skipped", "duration_sec": 0.0},
            ],
        }
        md = mod.render_markdown(report)
        self.assertIn("| Case | Status | Duration(s) |", md)
        self.assertIn("ollama_smoke", md)
        self.assertIn("zhipu_smoke", md)


if __name__ == "__main__":
    unittest.main()

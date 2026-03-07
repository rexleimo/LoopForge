use super::*;

pub(super) struct EnvVarGuard {
    key: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
    pub(super) fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = std::env::var_os(key);
        std::env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

pub(super) fn stub_bridge_script() -> &'static str {
    r#"import argparse
import json
import sys

PNG_B64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMB/6X9Yt8AAAAASUVORK5CYII="

parser = argparse.ArgumentParser()
parser.add_argument("--headless", action="store_true", default=True)
parser.add_argument("--no-headless", dest="headless", action="store_false")
parser.add_argument("--width", type=int, default=1280)
parser.add_argument("--height", type=int, default=720)
parser.add_argument("--timeout", type=int, default=30)
args = parser.parse_args()
headless = bool(args.headless)

sys.stdout.write(json.dumps({"success": True, "data": {"status": "ready"}}) + "\n")
sys.stdout.flush()

current_url = ""
history = []
scroll_x = 0
scroll_y = 0

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    cmd = json.loads(line)
    action = cmd.get("action", "")
    if action == "Navigate":
        current_url = cmd.get("url", "")
        history.append(current_url)
        resp = {"success": True, "data": {"title": "Stub", "url": current_url, "headless": headless}}
    elif action == "Back":
        if len(history) >= 2:
            history.pop()
            current_url = history[-1]
        resp = {"success": True, "data": {"title": "Stub", "url": current_url}}
    elif action == "Scroll":
        direction = cmd.get("direction", "down")
        amount = int(cmd.get("amount") or 0)
        if direction == "down":
            scroll_y += amount
        elif direction == "up":
            scroll_y -= amount
        elif direction == "right":
            scroll_x += amount
        elif direction == "left":
            scroll_x -= amount
        resp = {"success": True, "data": {"scrollX": scroll_x, "scrollY": scroll_y}}
    elif action == "ReadPage":
        resp = {"success": True, "data": {"title": "Stub", "url": current_url, "content": "hello"}}
    elif action == "Screenshot":
        resp = {"success": True, "data": {"format": "png", "url": current_url, "image_base64": PNG_B64}}
    elif action == "Click":
        resp = {"success": True, "data": {"clicked": cmd.get("selector", "")}}
    elif action == "Type":
        resp = {"success": True, "data": {"typed": cmd.get("text", ""), "selector": cmd.get("selector", "")}}
    elif action == "PressKey":
        resp = {"success": True, "data": {"key": cmd.get("key", ""), "selector": cmd.get("selector", "")}}
    elif action == "WaitFor":
        waited_for = {}
        if cmd.get("selector"):
            waited_for["selector"] = cmd.get("selector", "")
        if cmd.get("text"):
            waited_for["text"] = cmd.get("text", "")
        resp = {"success": True, "data": {"waited_for": waited_for, "timeout_ms": cmd.get("timeout_ms")}}
    elif action == "RunJs":
        resp = {"success": True, "data": {"result": 2, "expression": cmd.get("expression", "")}}
    elif action == "Close":
        resp = {"success": True, "data": {"status": "closed"}}
        sys.stdout.write(json.dumps(resp) + "\n")
        sys.stdout.flush()
        break
    else:
        resp = {"success": False, "error": "unknown action"}

    sys.stdout.write(json.dumps(resp) + "\n")
    sys.stdout.flush()
"#
}

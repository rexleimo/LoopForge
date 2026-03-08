use std::path::PathBuf;

use super::super::{CheckStatus, DoctorCheck};

pub(super) async fn push_browser_checks(checks: &mut Vec<DoctorCheck>, http: &reqwest::Client) {
    if let Ok(cdp) = std::env::var("LOOPFORGE_BROWSER_CDP_HTTP") {
        let cdp = cdp.trim().to_string();
        if !cdp.is_empty() {
            let probe = format!("{}/json/version", cdp.trim_end_matches('/'));
            match http.get(&probe).send().await {
                Ok(response) if response.status().is_success() => checks.push(DoctorCheck {
                    id: "browser.cdp_http".to_string(),
                    status: CheckStatus::Ok,
                    message: format!("GET {probe} -> {}", response.status()),
                }),
                Ok(response) => checks.push(DoctorCheck {
                    id: "browser.cdp_http".to_string(),
                    status: CheckStatus::Warn,
                    message: format!("GET {probe} -> {}", response.status()),
                }),
                Err(err) => checks.push(DoctorCheck {
                    id: "browser.cdp_http".to_string(),
                    status: CheckStatus::Warn,
                    message: format!("GET {probe} failed: {err}"),
                }),
            }
        }
        return;
    }

    match discover_chromium_executable() {
        Some(path) => checks.push(DoctorCheck {
            id: "browser.chromium".to_string(),
            status: CheckStatus::Ok,
            message: path.display().to_string(),
        }),
        None => checks.push(DoctorCheck {
            id: "browser.chromium".to_string(),
            status: CheckStatus::Warn,
            message: "chromium/chrome/edge not found; install a Chromium-based browser or set LOOPFORGE_BROWSER_CHROME_PATH (or use LOOPFORGE_BROWSER_CDP_HTTP)".to_string(),
        }),
    }
}

fn discover_chromium_executable() -> Option<PathBuf> {
    if let Ok(value) = std::env::var("LOOPFORGE_BROWSER_CHROME_PATH") {
        let path = PathBuf::from(value);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(path) = std::env::var("PATH") {
        let mut names: Vec<&str> = vec![
            "google-chrome",
            "chrome",
            "chromium",
            "chromium-browser",
            "microsoft-edge",
            "msedge",
            "brave",
            "brave-browser",
        ];
        if cfg!(windows) {
            names = vec!["chrome.exe", "msedge.exe", "brave.exe", "chromium.exe"];
        }

        for dir in std::env::split_paths(&path) {
            for name in &names {
                let candidate = dir.join(name);
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }

    if cfg!(windows) {
        let mut candidates: Vec<PathBuf> = Vec::new();
        for key in ["ProgramFiles", "ProgramFiles(x86)", "LocalAppData"] {
            if let Ok(base) = std::env::var(key) {
                let base = PathBuf::from(base);
                candidates.push(base.join("Google/Chrome/Application/chrome.exe"));
                candidates.push(base.join("Microsoft/Edge/Application/msedge.exe"));
                candidates.push(base.join("BraveSoftware/Brave-Browser/Application/brave.exe"));
            }
        }
        for candidate in candidates {
            if candidate.exists() {
                return Some(candidate);
            }
        }
    } else if cfg!(target_os = "macos") {
        let candidates = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
        ];
        for candidate in candidates {
            let path = PathBuf::from(candidate);
            if path.exists() {
                return Some(path);
            }
        }
    }

    None
}

use std::path::PathBuf;

pub(super) fn chromium_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();

    #[cfg(target_os = "macos")]
    {
        out.push("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome".into());
        out.push("/Applications/Chromium.app/Contents/MacOS/Chromium".into());
        out.push("/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge".into());
    }

    #[cfg(target_os = "linux")]
    {
        out.push("/usr/bin/google-chrome".into());
        out.push("/usr/bin/google-chrome-stable".into());
        out.push("/usr/bin/chromium".into());
        out.push("/usr/bin/chromium-browser".into());
    }

    #[cfg(windows)]
    {
        let program_files = std::env::var_os("ProgramFiles").map(PathBuf::from);
        let program_files_x86 = std::env::var_os("ProgramFiles(x86)").map(PathBuf::from);
        let local_app_data = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);

        for base in [program_files, program_files_x86, local_app_data] {
            let Some(base) = base else { continue };
            out.push(base.join("Google/Chrome/Application/chrome.exe"));
            out.push(base.join("Chromium/Application/chrome.exe"));
            out.push(base.join("Microsoft/Edge/Application/msedge.exe"));
        }
    }

    out
}

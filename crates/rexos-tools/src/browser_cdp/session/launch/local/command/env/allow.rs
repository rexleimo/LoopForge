pub(super) fn allowed_chromium_env_keys() -> &'static [&'static str] {
    &[
        "PATH",
        "HOME",
        "USER",
        "USERPROFILE",
        "SYSTEMROOT",
        "TEMP",
        "TMP",
        "TMPDIR",
        "APPDATA",
        "LOCALAPPDATA",
        "XDG_CONFIG_HOME",
        "XDG_CACHE_HOME",
        "DISPLAY",
        "WAYLAND_DISPLAY",
    ]
}

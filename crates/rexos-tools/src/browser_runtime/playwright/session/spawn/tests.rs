use super::command::{bridge_headless_arg, bridge_viewport_args};

#[test]
fn bridge_headless_arg_tracks_requested_mode() {
    assert_eq!(bridge_headless_arg(true), "--headless");
    assert_eq!(bridge_headless_arg(false), "--no-headless");
}

#[test]
fn bridge_viewport_args_remain_stable() {
    assert_eq!(
        bridge_viewport_args(),
        ["--width", "1280", "--height", "720", "--timeout", "30"]
    );
}

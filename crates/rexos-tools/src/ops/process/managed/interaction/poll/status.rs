pub(super) fn alive_from_exit_code(exit_code: Option<i32>) -> bool {
    exit_code.is_none()
}

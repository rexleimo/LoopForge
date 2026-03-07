pub(super) fn docker_exec_result(output: std::process::Output, image: String) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    serde_json::json!({
        "exit_code": exit_code,
        "stdout": stdout,
        "stderr": stderr,
        "image": image,
        "workdir": "/workspace",
    })
    .to_string()
}

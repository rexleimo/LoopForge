pub(super) fn shell_command(command: &str) -> tokio::process::Command {
    if cfg!(windows) {
        let mut cmd = tokio::process::Command::new("powershell");
        cmd.args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
        ]);

        let wrapped = format!(
            "$ErrorActionPreference = 'Stop'; $global:LASTEXITCODE = 0; {command}; if ($global:LASTEXITCODE -ne 0) {{ exit $global:LASTEXITCODE }}",
            command = command
        );
        cmd.arg(wrapped);
        cmd
    } else {
        let mut cmd = tokio::process::Command::new("bash");
        cmd.arg("-c").arg(command);
        cmd
    }
}

pub(super) fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

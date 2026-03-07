use std::process::Stdio;

pub(super) fn docker_exec_command(
    image: &str,
    mount: &str,
    command: &str,
) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("docker");
    cmd.arg("run")
        .arg("--rm")
        .arg("-i")
        .arg("--network")
        .arg("none")
        .arg("-v")
        .arg(mount)
        .arg("-w")
        .arg("/workspace")
        .arg(image)
        .arg("sh")
        .arg("-lc")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd
}

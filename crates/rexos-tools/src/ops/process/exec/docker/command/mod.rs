mod build;
mod output;
#[cfg(test)]
mod tests;

pub(super) fn docker_exec_command(
    image: &str,
    mount: &str,
    command: &str,
) -> tokio::process::Command {
    build::docker_exec_command(image, mount, command)
}

pub(super) fn docker_exec_result(output: std::process::Output, image: String) -> String {
    output::docker_exec_result(output, image)
}

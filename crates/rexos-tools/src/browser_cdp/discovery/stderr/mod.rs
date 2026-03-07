mod parse;
mod read;
mod tail;
#[cfg(test)]
mod tests;

use anyhow::{bail, Context};
use tokio::io::AsyncBufReadExt;

pub(crate) async fn read_devtools_url(
    stderr: tokio::process::ChildStderr,
) -> anyhow::Result<String> {
    let reader = tokio::io::BufReader::new(stderr);
    let mut lines = reader.lines();
    let deadline = read::stderr_deadline();
    let mut tail = tail::stderr_tail();

    loop {
        let line = match read::next_stderr_line(deadline, &mut lines).await {
            Ok(line) => line.context("read Chromium stderr")?,
            Err(_) => bail!(
                "{}",
                tail::timeout_error(&tail, "timed out waiting for Chromium to start")
            ),
        };

        match line {
            Some(line) => match parse::devtools_url_from_line(&line)? {
                Some(url) => return Ok(url),
                None => tail::push_stderr_line(&mut tail, line),
            },
            None => bail!(
                "{}",
                tail::timeout_error(&tail, "Chromium exited before printing DevTools URL")
            ),
        }
    }
}

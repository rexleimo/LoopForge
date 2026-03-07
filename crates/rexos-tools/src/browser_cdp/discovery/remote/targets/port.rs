use std::net::TcpListener;

use anyhow::Context;

pub(crate) fn pick_unused_port() -> anyhow::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0").context("bind 127.0.0.1:0")?;
    let port = listener.local_addr().context("local_addr")?.port();
    Ok(port)
}

mod close;
mod launch;
mod navigate;
#[cfg(test)]
mod tests;

fn resolved_headless(headless: Option<bool>, default_headless: bool) -> bool {
    headless.unwrap_or(default_headless)
}

mod field;
mod script;
mod shared;
#[cfg(test)]
mod tests;

fn validate_browser_expression(expression: &str) -> anyhow::Result<()> {
    shared::validate_browser_expression(expression)
}

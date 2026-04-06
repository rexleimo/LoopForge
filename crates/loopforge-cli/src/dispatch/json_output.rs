use serde::Serialize;

pub(crate) fn to_json_value<T: Serialize + ?Sized>(value: &T) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::to_value(value)?)
}

pub(crate) fn print_pretty_json<T: Serialize + ?Sized>(value: &T) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

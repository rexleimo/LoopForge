use crate::runtime_env;

pub(super) fn run() -> anyhow::Result<()> {
    let paths = runtime_env::ensure_paths()?;
    runtime_env::open_memory(&paths)?;
    println!("Initialized {}", paths.base_dir.display());
    Ok(())
}

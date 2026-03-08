use std::fs;

use anyhow::Context;

use crate::paths::RexosPaths;

use super::{RexosConfig, SkillsConfig, SkillsConfigWrapper};

fn serialize_default_config(default_config: &RexosConfig) -> anyhow::Result<String> {
    let mut toml_str = toml::to_string_pretty(default_config).context("serialize config")?;
    if !toml_str.contains("[skills]") {
        let skills_toml = toml::to_string_pretty(&SkillsConfigWrapper {
            skills: SkillsConfig::default(),
        })
        .context("serialize skills config")?;
        toml_str.push('\n');
        toml_str.push_str(&skills_toml);
    }
    Ok(toml_str)
}

pub(super) fn ensure_default_config(paths: &RexosPaths) -> anyhow::Result<()> {
    let config_path = paths.config_path();
    if config_path.exists() {
        return Ok(());
    }

    let toml_str = serialize_default_config(&RexosConfig::default())?;
    fs::write(&config_path, toml_str)
        .with_context(|| format!("write config: {}", config_path.display()))?;
    Ok(())
}

pub(super) fn load_config(paths: &RexosPaths) -> anyhow::Result<RexosConfig> {
    let config_path = paths.config_path();
    let raw = fs::read_to_string(&config_path)
        .with_context(|| format!("read config: {}", config_path.display()))?;
    toml::from_str(&raw).context("parse config TOML")
}

pub(super) fn load_skills_config(paths: &RexosPaths) -> anyhow::Result<SkillsConfig> {
    let config_path = paths.config_path();
    let raw = fs::read_to_string(&config_path)
        .with_context(|| format!("read config: {}", config_path.display()))?;
    let wrapper: SkillsConfigWrapper = toml::from_str(&raw).context("parse skills config TOML")?;
    Ok(wrapper.skills)
}

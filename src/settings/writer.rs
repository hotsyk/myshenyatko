use anyhow::{Context, Result};
use std::process::Command;

use super::{SettingDef, SettingValue, ValueType};

pub fn write_setting(def: &SettingDef, value: &SettingValue) -> Result<()> {
    write_to_domain(def.domain, def.key, def.value_type, value)?;
    for mirror in def.mirror_domains {
        write_to_domain(mirror, def.key, def.value_type, value)?;
    }
    Ok(())
}

fn write_to_domain(domain: &str, key: &str, vtype: ValueType, value: &SettingValue) -> Result<()> {
    let (type_flag, str_value) = match value {
        SettingValue::Float(v) => ("-float", v.to_string()),
        SettingValue::Bool(v) => ("-bool", if *v { "TRUE" } else { "FALSE" }.to_string()),
        SettingValue::Int(v) => ("-int", v.to_string()),
        SettingValue::Str(v) => {
            if vtype == ValueType::Str {
                ("-string", v.clone())
            } else {
                anyhow::bail!("type mismatch: expected {vtype:?}, got string");
            }
        }
    };

    let output = Command::new("defaults")
        .arg("write")
        .arg(domain)
        .arg(key)
        .arg(type_flag)
        .arg(&str_value)
        .output()
        .context("failed to execute defaults write")?;

    if !output.status.success() {
        anyhow::bail!(
            "defaults write failed for {domain}.{key}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    Ok(())
}

use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::process::Command;

use super::{SettingDef, SettingValue, ValueType};

pub fn read_all(settings: &[SettingDef]) -> HashMap<String, SettingValue> {
    let mut values = HashMap::new();
    for def in settings {
        if let Ok(val) = read_setting(def) {
            values.insert(def.id.to_string(), val);
        }
    }
    values
}

pub fn available_setting_ids(settings: &[SettingDef]) -> HashSet<String> {
    let mut domain_cache: HashMap<&str, bool> = HashMap::new();
    let mut available = HashSet::new();
    for def in settings {
        let exists = *domain_cache
            .entry(def.domain)
            .or_insert_with(|| domain_exists(def.domain));
        if exists {
            available.insert(def.id.to_string());
        }
    }
    available
}

fn domain_exists(domain: &str) -> bool {
    Command::new("defaults")
        .arg("read")
        .arg(domain)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn read_setting(def: &SettingDef) -> Result<SettingValue> {
    let output = Command::new("defaults")
        .arg("read")
        .arg(def.domain)
        .arg(def.key)
        .output()
        .context("failed to execute defaults command")?;

    if !output.status.success() {
        anyhow::bail!(
            "defaults read failed for {}.{}: {}",
            def.domain,
            def.key,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
    parse_value(&raw, def.value_type)
}

fn parse_value(raw: &str, value_type: ValueType) -> Result<SettingValue> {
    match value_type {
        ValueType::Float => {
            let v: f64 = raw.parse().context("parsing float")?;
            Ok(SettingValue::Float(v))
        }
        ValueType::Bool => {
            let v = match raw {
                "1" | "true" | "yes" => true,
                "0" | "false" | "no" => false,
                _ => anyhow::bail!("unexpected bool value: {raw}"),
            };
            Ok(SettingValue::Bool(v))
        }
        ValueType::Int => {
            let v: i64 = raw.parse().context("parsing int")?;
            Ok(SettingValue::Int(v))
        }
        ValueType::Str => Ok(SettingValue::Str(raw.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_float() {
        assert_eq!(
            parse_value("2.5", ValueType::Float).unwrap(),
            SettingValue::Float(2.5)
        );
    }

    #[test]
    fn parse_bool_variants() {
        assert_eq!(
            parse_value("1", ValueType::Bool).unwrap(),
            SettingValue::Bool(true)
        );
        assert_eq!(
            parse_value("0", ValueType::Bool).unwrap(),
            SettingValue::Bool(false)
        );
        assert_eq!(
            parse_value("true", ValueType::Bool).unwrap(),
            SettingValue::Bool(true)
        );
    }

    #[test]
    fn parse_int() {
        assert_eq!(
            parse_value("42", ValueType::Int).unwrap(),
            SettingValue::Int(42)
        );
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            parse_value("WhenScrolling", ValueType::Str).unwrap(),
            SettingValue::Str("WhenScrolling".to_string())
        );
    }
}

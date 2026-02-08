use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use super::Profile;

fn profiles_dir() -> Result<PathBuf> {
    let config = dirs::config_dir().context("could not determine config directory")?;
    let dir = config.join("myshenyatko").join("profiles");
    fs::create_dir_all(&dir).context("could not create profiles directory")?;
    Ok(dir)
}

fn profile_path(name: &str) -> Result<PathBuf> {
    let sanitized: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    Ok(profiles_dir()?.join(format!("{sanitized}.json")))
}

pub fn save(profile: &Profile) -> Result<()> {
    let path = profile_path(&profile.name)?;
    let json = serde_json::to_string_pretty(profile)?;
    fs::write(&path, json).context("writing profile")?;
    Ok(())
}

pub fn load(name: &str) -> Result<Profile> {
    let path = profile_path(name)?;
    let json = fs::read_to_string(&path).context("reading profile")?;
    let profile: Profile = serde_json::from_str(&json)?;
    Ok(profile)
}

pub fn list() -> Result<Vec<String>> {
    let dir = profiles_dir()?;
    let mut names = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            if let Some(stem) = path.file_stem() {
                names.push(stem.to_string_lossy().to_string());
            }
        }
    }
    names.sort();
    Ok(names)
}

pub fn delete(name: &str) -> Result<()> {
    let path = profile_path(name)?;
    if path.exists() {
        fs::remove_file(&path).context("deleting profile")?;
    }
    Ok(())
}

pub fn export_json(name: &str) -> Result<String> {
    let profile = load(name)?;
    Ok(serde_json::to_string_pretty(&profile)?)
}

pub fn import_json(json: &str) -> Result<Profile> {
    let profile: Profile = serde_json::from_str(json)?;
    save(&profile)?;
    Ok(profile)
}

use std::path::PathBuf;
use std::str::FromStr;

use serde::Deserialize;

#[allow(dead_code)]
pub fn deserialize_path<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut path = None;
    let value = String::deserialize(deserializer)?;
    if !value.is_empty() {
        let buf = shellexpand::tilde(&value);
        path = PathBuf::from_str(&buf).ok();
    }
    Ok(path)
}

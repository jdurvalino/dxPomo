use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;

use crate::model::PomodoroLog;

const DIR_NAME: &str = ".dxPomo";
const FILE_NAME: &str = "log.json";

fn log_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    let mut path = PathBuf::from(home);
    path.push(DIR_NAME);

    fs::create_dir_all(&path)?;
    path.push(FILE_NAME);

    Ok(path)
}

pub fn save(entry: PomodoroLog) -> Result<()> {
    let path = log_path()?;

    let mut logs = load().unwrap_or_default();
    logs.push(entry);

    let json = serde_json::to_string_pretty(&logs)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn load() -> Result<Vec<PomodoroLog>> {
    let path = log_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;

    let logs = serde_json::from_str(&content)?;
    Ok(logs)
}

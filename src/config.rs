use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct PomodoroConfig {
    pub focus_minutes: u64,
    pub break_minutes: u64,
    pub cycles: u32,
}

impl Default for PomodoroConfig {
    fn default() -> Self {
        Self {
            focus_minutes: 25,
            break_minutes: 5,
            cycles: 4,
        }
    }
}

fn config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Unable to locate the home directory.");
    path.push(".dxpomo_config.json");
    path
}

pub fn load() -> PomodoroConfig {
    let path = config_path();

    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        PomodoroConfig::default()
    }
}

pub fn save(config: &PomodoroConfig) {
    let path = config_path();
    let data = serde_json::to_string_pretty(config).unwrap();
    fs::write(path, data).expect("Error saving configuration.");
}

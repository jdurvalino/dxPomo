use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PomodoroLog {
    pub started_at: DateTime<Local>,
    pub duration_minutes: u64,
}

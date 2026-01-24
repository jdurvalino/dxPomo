use crate::storage::file;
use crate::model::{PomodoroType,PomodoroLog};
use chrono::{DateTime, Local};


pub fn run() {
    let logs = match file::load() {
        Ok(l) => l,
        Err(_) => {
            println!("📭 No records found.");
            return;
        }
    };

    let mut focus_count = 0;
    let mut break_count = 0;
    let mut focus_minutes = 0;
    let mut break_minutes = 0;

    for log in logs {
        match log.kind {
            PomodoroType::Focus => {
                focus_count += 1;
                focus_minutes += log.duration_minutes;
            }
            PomodoroType::Break => {
                break_count += 1;
                break_minutes += log.duration_minutes;
            }
        }
    }

    let total_sessions = focus_count + break_count;

    println!();
    println!("📊 Pomodoro Statistics");
    println!("---------------------------");
    println!("🔴 Focus sessions: {}", focus_count);
    println!("⏱️ Minutes of focus: {}", focus_minutes);
    println!("🟢 Rest periods: {}", break_count);
    println!("🛋️ Rest minutes: {}", break_minutes);
    println!("📦 Total sessions: {}", total_sessions);
    println!();
}

pub fn save_focus_log(started_at: DateTime<Local>, duration_minutes: u64) {
    let log = PomodoroLog {
        kind: PomodoroType::Focus,
        started_at: started_at,
        duration_minutes,
    };

    if let Err(e) = file::save(log) {
        eprintln!("⚠️ Failed to save focus log: {}", e);
    }
} 

pub fn save_break_log(started_at: DateTime<Local>, duration_minutes: u64) {
    let log = PomodoroLog {
        kind: PomodoroType::Break,
        started_at: started_at,
        duration_minutes,
    };

    if let Err(e) = file::save(log) {
        eprintln!("⚠️ Failed to save break log: {}", e);
    }
}

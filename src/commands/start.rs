use chrono::Local;

use crate::model::{PomodoroLog, PomodoroType};
use crate::storage::file;
use crate::config;
use crate::timer;


pub fn run() {
    
    let minutes = config::load().focus_minutes;

    println!("🍅 Pomodoro started: {} minutes of focus", minutes);

    let completed = timer::run_with_cancel(minutes);

    if !completed {
        println!("\n⛔ Focus canceled.");
        return;
    }

    let log = PomodoroLog {
        kind: PomodoroType::Focus,
        started_at: Local::now(),
        duration_minutes: minutes,
    }; 

    
    if let Err(e) = file::save(log) {
        eprintln!("⚠️ Faill to save pomodoro log {}", e);
    }

    println!("\n✅ Pomodoro finished! Good work.")
}

use chrono::Local;

use crate::model::{PomodoroLog, PomodoroType};
use crate::storage::file;
use crate::config;
use crate::timer;


pub fn run() {
    
    let minutes = config::load().break_minutes; 

    println!("☕ Break started: {} minutes", minutes);

    let completed = timer::run_with_cancel(minutes);


    if !completed {
        println!("\n⛔ Break canceled.");
        return;
    }

    let log = PomodoroLog {
        kind: PomodoroType::Break,
        started_at: Local::now(), 
        duration_minutes: minutes
    };

    if let Err(e) = file::save(log) {
        eprintln!("⚠️ Fail to save break time: {}", e);
    }

    println!("\n✅ Break complete!");
}

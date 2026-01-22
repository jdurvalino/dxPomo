use std::time::{Duration, Instant};
use std::thread::sleep;

use chrono::Local;

use crate::model::{PomodoroLog, PomodoroType};
use crate::storage::file;
use crate::config;


pub fn run() {

    let cfg = config::load();
    let minutes = cfg.focus_minutes;

    println!("🍅 Pomodoro started: {} minutes of focus", minutes);

    let start_time = Local::now();
    let total_seconds = minutes * 60;
    let start = Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs();
        if elapsed >= total_seconds {
            break;
        }

        let remaining = total_seconds - elapsed;
        let minutes = remaining / 60;
        let seconds = remaining % 60;

        print!("\r⏳ Time remaining: {:02}:{:02}", minutes, seconds);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        sleep(Duration::from_secs(1));
    }

    let log = PomodoroLog {
        kind: PomodoroType::Focus,
        started_at: start_time,
        duration_minutes: minutes,
    }; 

    
    if let Err(e) = file::save(log) {
        eprintln!("⚠️ Faill to save pomodoro log {}", e);
    }

    println!("\n✅ Pomodoro finished! Good work.")
}

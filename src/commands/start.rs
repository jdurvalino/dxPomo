use std::time::{Duration, Instant};
use std::thread::sleep;

use chrono::Local;

use crate::model::PomodoroLog;
use crate::storage::file;


const POMODORO_MINUTES: u64 = 50;

pub fn run() {

    let start_time = Local::now();
    let total_seconds = POMODORO_MINUTES * 60;
    let start = Instant::now();

    println!("🍅 Pomodoro started: {} minutes of focus", POMODORO_MINUTES);

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
        started_at: start_time,
        duration_minutes: POMODORO_MINUTES,
    }; 

    
    if let Err(e) = file::save(log) {
        eprintln!("⚠️ Faill to save pomodoro log {}", e);
    }

    println!("\n✅ Pomodoro finished! Good work.")
}

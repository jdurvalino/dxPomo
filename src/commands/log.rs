use crate::storage::file;

pub fn run() {
    match file::load() {
        Ok(logs) if logs.is_empty() => {
            println!("📭 No Pomodoro time recorded yet.");
        }
        Ok(mut logs) => {
            logs.sort_by(|a, b| b.started_at.cmp(&a.started_at));

            println!("🍅 Pomodoro History:\n");

            for (i, log) in logs.iter().enumerate() {
                let time = log.started_at.format("%d/%m/%Y %H:%M");
                println!(
                    "{}. {} — {} mimutes",
                    i + 1,
                    time,
                    log.duration_minutes
                );
            }
        }
        Err(e) => {
            eprintln!("⚠️ Fail to load history: {}", e); 
        }
    }
}

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::signal;

const BAR_WIDTH: usize = 20;

pub fn run_with_cancel(minutes: u64) -> bool {

    let total_seconds = minutes * 60;

    for elapsed in 0..=total_seconds {

        if signal::is_cancelled(){
            return false;
        }

        let progress = elapsed as f64 / total_seconds as f64;
        let filled = (progress * BAR_WIDTH as f64).round() as usize;
        let empty = BAR_WIDTH - filled;

        let remaining = total_seconds - elapsed;
        let min = remaining / 60;
        let sec = remaining % 60;

        print!(
            "\r[{}{}] {:>3}%  ⏳ {:02}:{:02}",
            "█".repeat(filled),
            "░".repeat(empty),
            (progress * 100.0) as u64,
            min,
            sec
        );

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    println!();
    true
}

use chrono::Local;

use crate::config;
use crate::timer;
use crate::commands::stats;

pub fn start(auto: bool, cycles: u32) {
    let cfg = config::load();
    let started_at;

    if auto {
        if cycles == 0 {
            eprintln!("❌ Cycles must be greater than zero");
            return;
        }

        run_auto_sequence(cfg.focus_minutes, cfg.break_minutes, cycles);
    } else {
        println!("▶ Focus start {} minutes", cfg.focus_minutes);
        started_at = Local::now();
        if !timer::run_with_cancel(cfg.focus_minutes){
            println!("\n⛔ Focus canceled");
            return (); 
        }
        stats::save_focus_log(started_at, cfg.focus_minutes);
    }
}

fn run_auto_sequence(focus: u64, rest: u64, cycles: u32) {
    let mut current_cycle = 1;
    let mut started_at;
    let cfg = config::load();

    loop {
        println!("\n▶ Focus start {} minutes (Cycle {}/{})", cfg.focus_minutes, current_cycle, cycles);
        
        started_at = Local::now();
        if !timer::run_with_cancel(focus) {
            println!("\n⛔ Sequence canceled");
            break;
        }
        
        stats::save_focus_log(started_at, cfg.focus_minutes);

        println!("\n☕Break start {} minutes", cfg.break_minutes);

        if !timer::run_with_cancel(rest) {
            println!("\n⛔ Sequence canceled");
            break;
        }
        
        stats::save_break_log(started_at, cfg.break_minutes);

        if current_cycle == cycles {
            println!("\n🎉 All cycles complete! Good work");
            break;
        } else {
            current_cycle += 1;
        }
    }
}


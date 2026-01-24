use chrono::Local;
use crate::config;
use crate::timer;
use crate::commands::stats;


pub fn run() {
   
    let cfg = config::load();
    let started_at;

    println!("☕ Break start {} minutes", cfg.break_minutes);
    started_at = Local::now();
    if !timer::run_with_cancel(cfg.break_minutes){
        println!("\n⛔ Break canceled");
        return (); 
    }
    stats::save_break_log(started_at, cfg.break_minutes);
    println!("\n✅ Break complete!");
}

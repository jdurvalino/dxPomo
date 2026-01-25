use crate::config;

pub fn set_focus(minutes: u64) {
    let mut cfg = config::load();
    cfg.focus_minutes = minutes;
    config::save(&cfg);
    println!("⏱️  Focus duration set to {} minutes.", minutes);
}

pub fn set_break(minutes: u64) {
    let mut cfg = config::load();
    cfg.break_minutes = minutes;
    config::save(&cfg);
    println!("☕  Break duration set to {} minutes.", minutes);
}

pub fn set_cycles(cycles: u32) {
    if cycles == 0 {
        eprintln!("❌ The number of cycles must be greater than zero");
        return;
    }
    let mut config = config::load();
    config.cycles = cycles;
    config::save(&config);
    println!("∞ Automatic cycles set to {}", cycles);
}

pub fn show() {
    let cfg = config::load();

    println!();
    println!("⚙️  Current configuration");
    println!("-------------------------");
    println!("⏱️Focus: {} minutes", cfg.focus_minutes);
    println!("☕Break: {} minutes", cfg.break_minutes);
    println!("∞ Cycles: {}", cfg.cycles);
    println!();
}

# 🍅 dxPomo CLI

A command-line interface (CLI) application written in Rust for focus management in the Pomodoro style.

It is a simple, fast, and reliable tool for daily use in the terminal, with local persistence of completed cycles.

This application was built with the purpose of studying and practicing the Rust programming language.


---

## Usage

### Start focus
cargo run -- start

### Start break
cargo run -- break

### Configure duration
cargo run -- config focus 45  
cargo run -- config break 10

### View current configuration
cargo run -- config show

### View history
cargo run -- log

### View statistics
cargo run -- stats



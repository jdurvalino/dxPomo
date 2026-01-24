# 🍅 dxPomo CLI

A command-line interface (CLI) application written in Rust for focus management in the Pomodoro style.

It is a simple, fast, and reliable tool for daily use in the terminal, with local persistence of completed cycles.

This application was built with the purpose of studying and practicing the Rust programming language.


---

## Usage

### Start focus
dxpomo start

![start](./img/start.jpg) 


### Start break
dxpomo break

![break](./img/break.jpg) 

### Configure duration
dxpomo config focus 50  

![confi focus](./img/config_focus.jpg) 


dxpomo config break 10

![config break](./img/config_break.jpg) 


### View current configuration
dxpomo config show

![config show](./img/config_show.jpg) 

### View history
dxpomo log

![log](./img/log.jpg) 

### View statistics
dxpomo stats

![log](./img/stats.jpg) 

### Cycles
dxpomo start --auto --cycles n

-- n = integer

![log](./img/auto.jpg) 



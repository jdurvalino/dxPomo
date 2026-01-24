# 🍅 dxPomo CLI

A command-line interface (CLI) application written in Rust for focus management in the Pomodoro style.

It is a simple, fast, and reliable tool for daily use in the terminal, with local persistence of completed cycles.

This application was built with the purpose of studying and practicing the Rust programming language.


---

## Usage


## Instalation

```bash
brew tap jdurvalino/dxpomo
brew install dxpomo
```


### Start focus
```bash
dxpomo start
```
![start](./img/start.jpg) 


### Start break
```bash
dxpomoo break
```

![break](./img/break.jpg) 

### Configure duration
``` bash
dxpomo config focus 50  
```

![confi focus](./img/config_focus.jpg) 

```bash
dxpomo config break 10
``` 

![config break](./img/config_break.jpg) 


### View current configuration
``` bash
dxpomo config show
``` 

![config show](./img/config_show.jpg) 

> [!TIP]
> The config are stored in a file called config.json, located in the .dxpomo in your Home directory.
> ~/.dxPomo/log.json 



### View history
``` bash 
dxpomo log
``` 

![log](./img/log.jpg) 

> [!TIP]
> The logs are stored in a file called log.json, located in the .dxpomo in your Home directory.
> ~/.dxPomo/log.json 



### View statistics
```bash 
dxpomo stats
```

![log](./img/stats.jpg) 

### Cycles
``` bash 
dxpomo start --auto --cycles n
```

![log](./img/auto.jpg) 



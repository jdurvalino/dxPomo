# 🗺️ dxPomo Roadmap

This document outlines the planned improvements and features for upcoming releases of **dxPomo**.

The roadmap is subject to change based on feedback, usage patterns, and project evolution.

---

## 🚀 Planned Features

### 1️⃣ Automatic cycle configuration
Allow users to define the number of Pomodoro cycles directly in the configuration file.

**Example:**
```json
{
  "auto_cycles": 4
}
``` 

and allow
```bash 
dxpomo start --auto
``` 

### 2️⃣ macOS Notifications

Display native macOS notifications when:
- A focus session ends
- A break session ends
- An automatic cycle sequence is completed

Notifications will be **optional** and **configurable**, allowing users to enable or disable them according to their workflow.


### Break session confirmation
Before starting a break session, prompt the user for confirmation:

```
Focus session completed.
Start break now? (y/n)
```


📌 Notes

All features will prioritize simplicity and terminal-first usability.
Backward compatibility will be preserved whenever possible.
Improvements may be released incrementally across minor versions.


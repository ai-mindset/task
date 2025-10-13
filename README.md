# task

A simple Markdown-based task management CLI, rewritten in Rust for improved
performance and reliability.

## Overview

`task` helps you manage tasks with due dates, creation dates, and completion
status in a Markdown file. Tasks can be added, marked as complete, cancelled,
and filtered by date -all from your terminal.

## Installation

```console
# Install from source
cargo install --git https://github.com/yourusername/task

# Or download binary from releases
# Then make it executable and move to your PATH
chmod +x task
sudo mv task /usr/local/bin/
```

## Usage

```console
# Add a task due today
task add "Buy groceries"

# Add a task with specific date
task add 2025-09-15 "Finish project"

# List pending tasks
task pending

# Mark task #2 as complete
task done 2

# View tasks due this week
task week

# Use a custom task file location
TASK_FILE=~/my-tasks.md task add "Custom location task"
```

### Task File Location

By default, tasks are stored in:
- Linux/macOS: `~/.task/work_log.md`
- Windows: `C:\Users\<YourUsername>\AppData\Local\Task\work_log.md`

You can customize this location by setting the `TASK_FILE` environment variable:

```console
# Linux/macOS
TASK_FILE=~/Documents/my-tasks.md task add "Use custom file"

# Windows (Command Prompt)
set TASK_FILE=C:\Users\YourUsername\Documents\my-tasks.md
task add "Use custom file"

# Windows (PowerShell)
$env:TASK_FILE="C:\Users\YourUsername\Documents\my-tasks.md"
task add "Use custom file"
```

For persistent settings:

```console
# Linux/macOS (.bashrc, .zshrc, etc.)
export TASK_FILE=~/Documents/work-tasks.md

# Windows (System Environment Variables)
# Right-click on This PC > Properties > Advanced System Settings > Environment Variables
# Add TASK_FILE as a user variable
```

## Features

- Markdown storage (human-readable, version control friendly)
- Due dates, creation dates, and completion/cancellation tracking
- Date-based filtering (today, this week, completed in past X weeks)
- Simple CLI interface with shortcuts (t=today, p=pending)
- Atomic file operations for data safety
- Cross-platform support (Linux, macOS, Windows)
- Configurable storage location via TASK_FILE environment variable

## Commands

| Command             | Alias | Description                           |
| ------------------- | ----- | ------------------------------------- |
| `add [date] <text>` | `a`   | Add task (with optional due date)     |
| `today`             | `t`   | List tasks due today                  |
| `week`              | `w`   | List tasks due in next 7 days         |
| `lastweek [weeks]`  | `lw`  | List tasks completed in last X weeks  |
| `pending`           | `p`   | List pending tasks                    |
| `done [num]`        | `d`   | Mark task complete or list completed  |
| `cancel [num]`      | `c`   | Mark task cancelled or list cancelled |
| `all`               | `l`   | List all tasks                        |

## License

MIT License

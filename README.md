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
```

## Features

- Markdown storage (human-readable, version control friendly)
- Due dates, creation dates, and completion/cancellation tracking
- Date-based filtering (today, this week, completed in past X weeks)
- Simple CLI interface with shortcuts (t=today, p=pending)
- Atomic file operations for data safety

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

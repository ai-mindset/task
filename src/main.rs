use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use chrono::{Duration, Local, NaiveDate};
use clap::{Parser, Subcommand};
use once_cell::sync::Lazy;
use regex::Regex;

// Regex patterns for date extraction
static DUE_DATE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"📅\s+(\d{4}-\d{2}-\d{2})").unwrap());
static COMPLETION_DATE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"✅\s+(\d{4}-\d{2}-\d{2})").unwrap());
static DATE_PART_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(📅[^📋]*📋[^\s]*)").unwrap());

#[derive(Parser)]
#[command(name = "task")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "a")]
    Add {
        date: Option<String>,
        text: Vec<String>,
    },

    #[command(alias = "t")]
    Today,

    #[command(alias = "w")]
    Week,

    #[command(alias = "lw")]
    LastWeek {
        #[arg(default_value = "1")]
        weeks: u32,
    },

    #[command(alias = "p")]
    Pending,

    #[command(alias = "d")]
    Done { task_nums: Vec<usize> },

    #[command(alias = "c")]
    Cancel { task_num: Option<usize> },

    #[command(alias = "l", alias = "list")]
    All,
}

fn get_task_file() -> PathBuf {
    env::var("TASK_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").unwrap_or_else(|_| String::from("."));
            PathBuf::from(home).join("Documents/work_log.md")
        })
}

fn read_lines(path: &PathBuf) -> Vec<String> {
    if !path.exists() {
        File::create(path).unwrap();
    }
    BufReader::new(File::open(path).unwrap())
        .lines()
        .collect::<Result<_, _>>()
        .unwrap()
}

fn write_lines(path: &PathBuf, lines: &[String]) {
    let temp_path = path.with_extension("tmp");
    let mut file = File::create(&temp_path).unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }
    file.sync_all().unwrap();
    fs::rename(temp_path, path).unwrap();
}

fn extract_date(line: &str, regex: &Regex) -> Option<NaiveDate> {
    regex
        .captures(line)
        .and_then(|cap| cap.get(1))
        .and_then(|m| NaiveDate::parse_from_str(m.as_str(), "%Y-%m-%d").ok())
}

fn print_header() {
    println!("📝 SIMPLE TASK MANAGER 📝");
    println!("==========================\n");
    println!(
        "Emoji Legend: 📅 = Due date   📋 = Creation date   ✅ = Completion date   ❌ = Cancellation date\n"
    );
}

fn main() {
    let cli = Cli::parse();
    let task_file = get_task_file();

    print_header();

    match cli.command {
        Some(Commands::Add { date, text }) => {
            // Get today's date
            let today = Local::now().date_naive().format("%Y-%m-%d").to_string();

            // Determine due date and task text
            let (due_date, task_text) = match date {
                // Date parameter is provided
                Some(d) => {
                    // Check if it's a properly formatted date
                    if d.len() == 10 && d.chars().nth(4) == Some('-') {
                        // It's a valid date format
                        (d, text.join(" "))
                    } else {
                        // Not a date - it's actually part of the task text
                        // Prepend it to the rest of the text
                        let mut full_text = vec![d];
                        full_text.extend(text);
                        (today.clone(), full_text.join(" "))
                    }
                }
                // No date parameter, just use today's date
                None => (today.clone(), text.join(" ")),
            };

            // Validate the task text
            if task_text.is_empty() {
                eprintln!("Error: Task cannot be empty.");
                return;
            }

            let task_line = format!("- [ ] 📅 {} 📋 {} {}", due_date, today, task_text);
            let mut lines = read_lines(&task_file);
            lines.push(task_line);
            write_lines(&task_file, &lines);
            println!("Added task due 📅 {}: {}", due_date, task_text);
        }

        Some(Commands::Today) => {
            let today = Local::now().date_naive().format("%Y-%m-%d").to_string();
            println!("Tasks due today (📅 {}):", today);
            let lines = read_lines(&task_file);
            let mut found = false;

            for (i, line) in lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.contains("- [ ]"))
            {
                if let Some(cap) = DUE_DATE_RE.captures(line) {
                    if cap.get(1).map_or("", |m| m.as_str()) == today {
                        println!("{} - {}", i + 1, line);
                        found = true;
                    }
                }
            }
            if !found {
                println!("No tasks due today.");
            }
        }

        Some(Commands::Week) => {
            let today = Local::now().date_naive();
            let week_later = today + Duration::days(7);
            println!("Tasks due in the next 7 days:");
            let lines = read_lines(&task_file);
            let mut found = false;

            for (i, line) in lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.contains("- [ ]"))
            {
                if let Some(due_date) = extract_date(line, &DUE_DATE_RE) {
                    if due_date >= today && due_date <= week_later {
                        println!("{} - {}", i + 1, line);
                        found = true;
                    }
                }
            }
            if !found {
                println!("No tasks due this week.");
            }
        }

        Some(Commands::LastWeek { weeks }) => {
            let today = Local::now().date_naive();
            let weeks_ago = today - Duration::days(7 * weeks as i64);
            println!("Tasks completed in the last {} week(s):", weeks);
            let lines = read_lines(&task_file);
            let mut found = false;

            for (i, line) in lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.contains("- [x]"))
            {
                if let Some(completion_date) = extract_date(line, &COMPLETION_DATE_RE) {
                    if completion_date >= weeks_ago && completion_date <= today {
                        println!("{} - {}", i + 1, line);
                        found = true;
                    }
                }
            }
            if !found {
                println!("No tasks completed in the last {} week(s).", weeks);
            }
        }

        Some(Commands::Pending) => {
            println!("Pending tasks:");
            let lines = read_lines(&task_file);
            let pending = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.contains("- [ ]"))
                .collect::<Vec<_>>();

            if pending.is_empty() {
                println!("No pending tasks.");
            } else {
                for (i, (_, line)) in pending.iter().enumerate() {
                    println!("{} - {}", i + 1, line);
                }
            }
        }

        Some(Commands::Done { task_nums }) => {
            let mut lines = read_lines(&task_file);

            if task_nums.is_empty() {
                println!("Completed tasks:");
                let completed = lines
                    .iter()
                    .enumerate()
                    .filter(|(_, l)| l.contains("- [x]"))
                    .collect::<Vec<_>>();

                if completed.is_empty() {
                    println!("No completed tasks.");
                } else {
                    for (i, (_, line)) in completed.iter().enumerate() {
                        println!("{} - {}", i + 1, line);
                    }
                }
                return;
            }

            let pending = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.contains("- [ ]"))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            let completion_date = Local::now().date_naive().format("%Y-%m-%d").to_string();

            for &task_num in &task_nums {
                if task_num == 0 || task_num > pending.len() {
                    println!(
                        "Error: Task number out of range. Run 'task pending' to see available tasks."
                    );
                    continue;
                }

                let line_idx = pending[task_num - 1];
                lines[line_idx] =
                    lines[line_idx].replace("- [ ]", &format!("- [x] ✅ {}", completion_date));
                println!("Task {} marked as completed", task_num);
            }

            write_lines(&task_file, &lines);
        }

        Some(Commands::Cancel { task_num }) => {
            let mut lines = read_lines(&task_file);

            if task_num.is_none() {
                println!("Cancelled tasks:");
                let cancelled = lines
                    .iter()
                    .enumerate()
                    .filter(|(_, l)| l.contains("- [-] ❌"))
                    .collect::<Vec<_>>();

                if cancelled.is_empty() {
                    println!("No cancelled tasks.");
                } else {
                    for (i, (_, line)) in cancelled.iter().enumerate() {
                        println!("{} - {}", i + 1, line);
                    }
                }
                return;
            }

            let task_num = task_num.unwrap();
            let pending = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.contains("- [ ]"))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            if task_num == 0 || task_num > pending.len() {
                eprintln!(
                    "Error: Task number out of range. Run 'task pending' to see available tasks."
                );
                return;
            }

            let line_idx = pending[task_num - 1];
            let line = &lines[line_idx];

            let date_part = DATE_PART_RE
                .captures(line)
                .and_then(|cap| cap.get(1))
                .map_or("", |m| m.as_str());

            let task_text = line.splitn(2, date_part).nth(1).unwrap_or("").trim();

            let cancellation_date = Local::now().date_naive().format("%Y-%m-%d").to_string();
            lines[line_idx] = format!(
                "- [-] ❌ {} {} ~~{}~~",
                cancellation_date, date_part, task_text
            );

            write_lines(&task_file, &lines);
            println!("Task {} marked as cancelled", task_num);
        }

        Some(Commands::All) => {
            println!("All tasks:");
            let lines = read_lines(&task_file);

            if lines.is_empty() {
                println!("No tasks found.");
            } else {
                for (i, line) in lines.iter().enumerate() {
                    println!("{} - {}", i + 1, line);
                }
            }
        }

        None => {
            println!("Usage: task [command] [args]");
            println!("Commands:");
            println!(
                "  add|a [date] \"<text>\"  Add a new task with optional due date (YYYY-MM-DD), defaults to today"
            );
            println!("  today|t              List tasks due today");
            println!("  week|w               List tasks due in the next 7 days");
            println!(
                "  lastweek|lw [weeks]  List tasks completed in the last X weeks (default: 1)"
            );
            println!("  pending|p            List all pending tasks");
            println!("  done|d [num]         Mark task as complete or list completed tasks");
            println!("  cancel|c [num]       Mark task as cancelled or list cancelled tasks");
            println!("  all|list|l           List all tasks");
            println!("");
            println!("Examples:");
            println!("  task add \"Buy groceries\"                 # Add task due today");
            println!("  task add 2025-09-15 \"Finish project\"     # Add task with due date");
            println!("  task pending                            # List pending tasks");
            println!("  task done 2                             # Mark task #2 as complete");
        }
    }
}

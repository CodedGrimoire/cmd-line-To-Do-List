use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use serde_json;
use clap::{Arg, Command};
use chrono::{NaiveDate, Utc}; // For deadline and date handling
use std::collections::HashSet; // For tags

#[derive(Serialize, Deserialize, Debug, Clone)] 
struct Task {
    description: String,
    completed: bool,
    priority: u8, // Priority field (1-5)
    due_date: Option<NaiveDate>, // Due date for tasks
    tags: HashSet<String>, // Tags for categorizing tasks
    recurrence: Option<String>, // Recurrence frequency (daily, weekly, etc.)
    depends_on: Option<usize>, // Task dependencies (index of the task this depends on)
}

const FILE_PATH: &str = "tasks.json";

fn main() {
    let matches = Command::new("Rust To-Do CLI")
        .version("1.0")
        .author("Your Name")
        .about("Manage your tasks from the terminal")
        .subcommand(Command::new("add")
            .about("Add a new task")
            .arg(Arg::new("task")
                .required(true)
                .help("The task description"))
            .arg(Arg::new("priority")
                .required(true)
                .help("Priority of the task (1-5)"))
            .arg(Arg::new("due_date")
                .required(false)
                .help("Due date for the task in format YYYY-MM-DD"))
            .arg(Arg::new("tags")
                .required(false)
                .help("Comma separated tags for the task"))
            .arg(Arg::new("recurrence")
                .required(false)
                .help("Recurrence of the task (e.g., 'daily', 'weekly')")))
        .subcommand(Command::new("list").about("List all tasks"))
        .subcommand(Command::new("complete")
            .about("Mark a task as completed")
            .arg(Arg::new("index")
                .required(true)
                .help("The task index to mark as completed")))
        .subcommand(Command::new("completed")
            .about("Display the number of completed tasks"))
        .subcommand(Command::new("search")
            .about("Search for tasks by keyword")
            .arg(Arg::new("keyword")
                .required(true)
                .help("The keyword to search for in task descriptions")))
        .get_matches();

        if matches.subcommand_name().is_none() {
            println!("Available commands:");
            println!("  add <task_description> <priority>  - Add a new task with priority (1-5)");
            println!("  list                               - List all tasks, sorted by priority");
            println!("  complete <task_index>              - Mark a task as completed");
            println!("  completed                         - Show the number of completed tasks");
            println!("  search <keyword>                  - Search for tasks by keyword");
            println!("  filter <status>                   - Filter tasks by completion status (completed/incomplete)");
            println!("  due <date>                        - List tasks by due date (YYYY-MM-DD)");
            println!("  overdue                           - List tasks that are overdue");
            return;
        }
        

    let mut tasks = load_tasks();

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(task_desc) = matches.get_one::<String>("task") {
            if let Some(priority_str) = matches.get_one::<String>("priority") {
                if let Ok(priority) = priority_str.parse::<u8>() {
                    if priority >= 1 && priority <= 5 {
                        let due_date = matches.get_one::<String>("due_date").and_then(|date_str| {
                            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
                        });

                        let tags = matches.get_one::<String>("tags")
                            .map(|tags_str| tags_str.split(',').map(|s| s.trim().to_string()).collect::<HashSet<String>>())
                            .unwrap_or_else(HashSet::new);

                        let recurrence = matches.get_one::<String>("recurrence").map(|rec| rec.to_string());

                        tasks.push(Task {
                            description: task_desc.clone(),
                            completed: false,
                            priority,
                            due_date,
                            tags,
                            recurrence,
                            depends_on: None, // No dependency by default
                        });
                        save_tasks(&tasks);
                        println!("Task added: '{}', Priority: {}, Due: {:?}", task_desc, priority, due_date);
                    } else {
                        println!("Priority must be between 1 and 5.");
                    }
                } else {
                    println!("Invalid priority value.");
                }
            }
        }
    } else if matches.subcommand_matches("list").is_some() {
        let mut sorted_tasks = tasks.clone();
        sorted_tasks.sort_by_key(|task| task.priority);

        for (i, task) in sorted_tasks.iter().enumerate() {
            let status = if task.completed { "✔" } else { "✘" };
            println!("{}: [{}] [{}] {} Due: {:?} Tags: {:?} Recurrence: {:?}",
                     i, status, task.priority, task.description, task.due_date, task.tags, task.recurrence);
        }
    } else if let Some(matches) = matches.subcommand_matches("complete") {
        if let Some(index_str) = matches.get_one::<String>("index") {
            if let Ok(index) = index_str.parse::<usize>() {
                if index < tasks.len() {
                    tasks[index].completed = true;
                    save_tasks(&tasks);
                    println!("Task marked as completed: {}", tasks[index].description);
                } else {
                    println!("Invalid task index.");
                }
            }
        }
    } else if matches.subcommand_matches("completed").is_some() {
        let completed_count = tasks.iter().filter(|task| task.completed).count();
        println!("Completed tasks: {}/{}", completed_count, tasks.len());
    } else if let Some(matches) = matches.subcommand_matches("search") {
        if let Some(keyword) = matches.get_one::<String>("keyword") {
            for (i, task) in tasks.iter().enumerate() {
                if task.description.contains(keyword) {
                    println!("{}: [{}] [{}] {}", i, task.completed, task.priority, task.description);
                }
            }
        }
    }
}

fn load_tasks() -> Vec<Task> {
    let file_content = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(FILE_PATH, json).expect("Failed to save tasks");
}

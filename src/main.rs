use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs;
use chrono::Local;

// Import the modules
mod gui;
mod models;
mod ui;
mod utils;

// Re-export Task from main.rs since it's referenced by other modules
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub priority: u8,
    pub deadline: Option<i64>,  // UNIX timestamp for deadline
}

const FILE_PATH: &str = "tasks.json";

// Function to load tasks from the JSON file
pub fn load_tasks() -> Vec<Task> {
    let file_content = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
}

// Function to save tasks to the JSON file
pub fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(FILE_PATH, json).expect("Failed to save tasks");
}

fn main() {
    let matches = Command::new("TaskForge")
        .version("2.0")
        .author("Your Name")
        .about("Manage your tasks efficiently from terminal or GUI")
        .subcommand(Command::new("add")
            .about("Add a new task")
            .arg(Arg::new("task")
                .required(true)
                .help("The task description"))
            .arg(Arg::new("priority")
                .required(true)
                .help("Priority of the task (1-5)"))
            .arg(Arg::new("deadline")
                .required(false)
                .help("Optional deadline in YYYY-MM-DD format")))
        .subcommand(Command::new("list").about("List all tasks"))
        .subcommand(Command::new("today").about("List tasks due today"))
        .subcommand(Command::new("complete")
            .about("Mark a task as completed")
            .arg(Arg::new("index")
                .required(true)
                .help("The task index to mark as completed")))
        .subcommand(Command::new("completed")
            .about("Display the number of completed tasks"))
        .subcommand(Command::new("gui")
            .about("Launch the GUI version of the app"))
        .get_matches();

    // If the "gui" command is passed, launch the GUI
    if let Some(_matches) = matches.subcommand_matches("gui") {
        // Run the GUI application with egui
        if let Err(e) = gui::TodoAppGUI::run(None) {
            eprintln!("Failed to run GUI: {}", e);
        }
        return;
    }

    // Otherwise, handle CLI commands
    let mut tasks = load_tasks();

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(task_desc) = matches.get_one::<String>("task") {
            if let Some(priority_str) = matches.get_one::<String>("priority") {
                if let Ok(priority) = priority_str.parse::<u8>() {
                    if priority >= 1 && priority <= 5 {
                        // Parse optional deadline if provided
                        let deadline = matches.get_one::<String>("deadline").and_then(|date_str| {
                            // Parse YYYY-MM-DD format
                            match chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                                Ok(date) => {
                                    // Create a datetime with end of day (23:59:59)
                                    let naive_datetime = date.and_hms_opt(23, 59, 59).unwrap();
                                    // Convert to timestamp
                                    Some(naive_datetime.and_utc().timestamp())
                                },
                                Err(_) => {
                                    println!("Warning: Could not parse deadline '{}'. Format should be YYYY-MM-DD.", date_str);
                                    None
                                }
                            }
                        });
                        
                        tasks.push(Task {
                            description: task_desc.clone(),
                            completed: false,
                            priority,
                            deadline,
                        });
                        
                        save_tasks(&tasks);
                        
                        let deadline_msg = if let Some(_) = deadline {
                            format!(" with deadline {}", matches.get_one::<String>("deadline").unwrap())
                        } else {
                            "".to_string()
                        };
                        
                        println!("Task added: '{}', Priority: {}{}", task_desc, priority, deadline_msg);
                    } else {
                        println!("Priority must be between 1 and 5.");
                    }
                } else {
                    println!("Invalid priority value.");
                }
            }
        }
    } else if matches.subcommand_matches("list").is_some() {
        // Sort tasks by priority (1 is most urgent)
        let mut sorted_tasks = tasks.clone();
        sorted_tasks.sort_by_key(|task| task.priority);

        for (i, task) in sorted_tasks.iter().enumerate() {
            let status = if task.completed { "✅" } else { "❌" };
            let deadline_str = match task.deadline {
                Some(timestamp) => {
                    let datetime = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
                    let local_datetime = chrono::DateTime::<Local>::from_naive_utc_and_offset(
                        datetime,
                        *Local::now().offset()
                    );
                    format!(" [Due: {}]", local_datetime.format("%Y-%m-%d %H:%M"))
                },
                None => "".to_string()
            };
            
            println!("{}: [{}] [P{}] {}{}", i, status, task.priority, task.description, deadline_str);
        }
    } else if matches.subcommand_matches("today").is_some() {
        // Filter tasks due today
        let today = Local::now().date_naive();
        let today_start = today.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
        let today_end = today.and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp();
        
        let today_tasks: Vec<&Task> = tasks.iter()
            .filter(|task| {
                if let Some(deadline) = task.deadline {
                    deadline >= today_start && deadline <= today_end
                } else {
                    false
                }
            })
            .collect();
            
        if today_tasks.is_empty() {
            println!("No tasks due today.");
        } else {
            println!("Tasks due today:");
            for (i, task) in today_tasks.iter().enumerate() {
                let status = if task.completed { "✅" } else { "❌" };
                println!("{}: [{}] [P{}] {}", i + 1, status, task.priority, task.description);
            }
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
    }
}
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::SystemTime;
use chrono::{NaiveDate, Local,Datelike};

#[derive(Debug, Clone, PartialEq)]
pub enum TaskAction {
    ToggleCompletion(usize),
    Delete(usize),
    SetDeadline(usize, Option<i64>), // UNIX timestamp
}

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum ViewTab {
    #[default]
    All,
    Today,
    Upcoming,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum SortMode {
    #[default]
    Added,
    Priority,
    Deadline,
}

// Application state
pub struct AppState {
    pub tasks: Vec<crate::Task>,
    pub new_task_description: String,
    pub new_task_priority: u8,
    pub new_task_deadline: Option<i64>,
    pub task_sender: Sender<Vec<crate::Task>>,
    pub task_receiver: Receiver<Vec<crate::Task>>,
    pub save_requested: bool,
    pub action: Option<TaskAction>,
    
    // Calendar related state
    pub selected_date: Option<NaiveDate>,
    pub calendar_month: u32,
    pub calendar_year: i32,
    
    // View options
    pub current_tab: ViewTab,
    pub sort_mode: SortMode,
    pub show_notifications: bool,
    pub show_calendar: bool,
    
    // Notification state
    pub notifications: Vec<String>,
    pub last_notification_check: SystemTime,
}

impl Default for AppState {
    fn default() -> Self {
        let (task_sender, task_receiver) = channel();
        
        let now = Local::now().naive_local();
        
        Self {
            tasks: Vec::new(),
            new_task_description: String::new(),
            new_task_priority: 2, // Medium priority as default
            new_task_deadline: None,
            task_sender,
            task_receiver,
            save_requested: false,
            action: None,
            
            // Calendar starts at current month and year
            selected_date: None,
            calendar_month: now.month(),
            calendar_year: now.year(),
            
            // Default view options
            current_tab: ViewTab::All,
            sort_mode: SortMode::Added,
            show_notifications: true,
            show_calendar: false,
            
            // Notification state
            notifications: Vec::new(),
            last_notification_check: SystemTime::now(),
        }
    }
}

impl AppState {
    // Method to create a new instance with fresh channels
    pub fn new() -> Self {
        Default::default()
    }
}
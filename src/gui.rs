use crate::{Task, save_tasks, load_tasks};
use crate::models::{AppState, SortMode, ViewTab, TaskAction};
use crate::ui;
use crate::ui::theme::*;
use eframe::egui;
use std::sync::mpsc::channel;
use std::time::SystemTime;

use chrono::{Local, Datelike};
pub struct TodoAppGUI {
    state: AppState,
}

impl TodoAppGUI {
    pub fn run(_flags: Option<()>) -> Result<(), eframe::Error> {
        let native_options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1000.0, 1000.0)),
            min_window_size: Some(egui::vec2(600.0, 500.0)),
            resizable: true,
            transparent: false,
            ..Default::default()
        };
        
        eframe::run_native(
            "TaskForge",
            native_options,
            Box::new(|cc| {
                // Set up custom theme
                ui::setup_theme(&cc.egui_ctx);
                
                Box::new(Self::new(cc))
            })
        )
    }
    
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load tasks
        let tasks = load_tasks();
        
        // Create channels for task updates
        let (task_sender, task_receiver) = channel();
        
        // Get current date for calendar
        let now = Local::now();
        
        let state = AppState {
            tasks,
            new_task_description: String::new(),
            new_task_priority: 3,
            new_task_deadline: None,
            task_sender,
            task_receiver,
            save_requested: false,
            action: None,
            selected_date: None,
            calendar_month: now.month(),
            calendar_year: now.year(),
            current_tab: ViewTab::All,
            sort_mode: SortMode::Deadline,
            show_notifications: true,
            show_calendar: true,
            notifications: Vec::new(),
            last_notification_check: SystemTime::now(),
        };
        
        Self { state }
    }
    
    fn add_task(&mut self) {
        if !self.state.new_task_description.trim().is_empty() {
            self.state.tasks.push(Task {
                description: self.state.new_task_description.clone(),
                completed: false,
                priority: self.state.new_task_priority,
                deadline: self.state.new_task_deadline,
            });
            
            // Reset input
            self.state.new_task_description.clear();
            self.state.new_task_deadline = None;
            
            // Sort tasks based on current sort mode
            self.sort_tasks();
            
            // Save tasks
            self.state.save_requested = true;
        }
    }
    
    fn delete_task(&mut self, index: usize) {
        if index < self.state.tasks.len() {
            // Print for debugging
            println!("Deleting task at index {}: {}", index, self.state.tasks[index].description);
            
            self.state.tasks.remove(index);
            self.state.save_requested = true;
        }
    }
    
    fn toggle_task_completion(&mut self, index: usize) {
        if index < self.state.tasks.len() {
            self.state.tasks[index].completed = !self.state.tasks[index].completed;
            self.state.save_requested = true;
        }
    }
    
    fn set_task_deadline(&mut self, index: usize, deadline: Option<i64>) {
        if index < self.state.tasks.len() {
            self.state.tasks[index].deadline = deadline;
            self.state.save_requested = true;
        }
    }
    
    fn sort_tasks(&mut self) {
        match self.state.sort_mode {
            SortMode::Priority => {
                self.state.tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
            },
            SortMode::Deadline => {
                self.state.tasks.sort_by(|a, b| {
                    match (a.deadline, b.deadline) {
                        (Some(a_deadline), Some(b_deadline)) => a_deadline.cmp(&b_deadline),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => a.priority.cmp(&b.priority),
                    }
                });
            },
            SortMode::Added => {
                // Already in order added, do nothing
            }
        }
    }
}

impl eframe::App for TodoAppGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for tasks with approaching deadlines
        if self.state.show_notifications {
            let tasks_clone = self.state.tasks.clone();
            ui::check_notifications(&mut self.state, &tasks_clone);
        }
        
        // Handle any pending actions from previous frame
        if let Some(action) = self.state.action.take() {
            println!("Processing action: {:?}", action);
            match action {
                TaskAction::ToggleCompletion(index) => self.toggle_task_completion(index),
                TaskAction::Delete(index) => self.delete_task(index),
                TaskAction::SetDeadline(index, deadline) => self.set_task_deadline(index, deadline),
            }
        }
        
        // Check for new tasks from the task_receiver
        if let Ok(new_tasks) = self.state.task_receiver.try_recv() {
            self.state.tasks = new_tasks;
        }
        
        // Save tasks if requested
        if self.state.save_requested {
            let tasks_to_save = self.state.tasks.clone();
            std::thread::spawn(move || {
                save_tasks(&tasks_to_save);
            });
            self.state.save_requested = false;
        }
        
        // Top panel for app title and notifications
        egui::TopBottomPanel::top("title_panel")
            .frame(egui::Frame::none()
                .fill(DARK_BG)
                .stroke(egui::Stroke::new(1.0, DARK_ACCENT))
                .inner_margin(egui::Margin::same(10.0)))
            .show(ctx, |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    // Pink dot for aesthetic
                    ui.add(egui::Label::new(
                        egui::RichText::new("●")
                            .color(PINK_PRIMARY)
                            .size(24.0)
                    ));
                    ui.add_space(8.0);
                    
                    // App title
                    ui.add(egui::Label::new(
                        egui::RichText::new("TaskForge")
                            .color(TEXT_COLOR)
                            .size(24.0)
                            .strong()
                    ));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Toggle notifications
                        let notif_text = if self.state.show_notifications { "🔔" } else { "🔕" };
                        if ui.button(notif_text).clicked() {
                            self.state.show_notifications = !self.state.show_notifications;
                        }
                        
                        ui.add_space(10.0);
                        
                        // Display date
                        let now = Local::now();
                        ui.label(egui::RichText::new(format!("{}", now.format("%A, %B %d, %Y")))
                            .color(MUTED_TEXT));
                    });
                });
                
                // Show notifications if enabled and present
                if self.state.show_notifications && !self.state.notifications.is_empty() {
                    ui::draw_notifications(ui, &self.state.notifications);
                }
                
                ui.add_space(5.0);
            });
        
        // Bottom panel for sorting options
        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(egui::Frame::none()
                .fill(DARK_BG)
                .inner_margin(egui::Margin::same(10.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Sort by:");
                    
                    if ui.selectable_label(self.state.sort_mode == SortMode::Priority, "Priority").clicked() {
                        self.state.sort_mode = SortMode::Priority;
                        self.sort_tasks();
                    }
                    
                    if ui.selectable_label(self.state.sort_mode == SortMode::Deadline, "Deadline").clicked() {
                        self.state.sort_mode = SortMode::Deadline;
                        self.sort_tasks();
                    }
                    
                    if ui.selectable_label(self.state.sort_mode == SortMode::Added, "Date Added").clicked() {
                        self.state.sort_mode = SortMode::Added;
                        self.sort_tasks();
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Toggle calendar view
                        let calendar_text = if self.state.show_calendar { "Hide Calendar" } else { "Show Calendar" };
                        if ui.button(calendar_text).clicked() {
                            self.state.show_calendar = !self.state.show_calendar;
                        }
                    });
                });
            });
        
        // Left panel for adding new tasks
        egui::SidePanel::left("add_task_panel")
            .frame(egui::Frame::none()
                .fill(DARK_BG)
                .inner_margin(egui::Margin::same(16.0)))
            .resizable(false)
            .default_width(280.0)
            .show(ctx, |ui| {
                // Add task panel
                ui::draw_create_task_panel(ui, &mut self.state);
                
                ui.add_space(20.0);
                
                // Visual divider
                let separator_color = DARK_ACCENT.linear_multiply(0.5);
                ui.painter().line_segment(
                    [
                        ui.min_rect().left_center() + egui::vec2(0.0, 10.0),
                        ui.min_rect().right_center() + egui::vec2(0.0, 10.0)
                    ],
                    egui::Stroke::new(1.0, separator_color)
                );
                
                ui.add_space(20.0);
                
                // View filters
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.state.current_tab == ViewTab::All, "All").clicked() {
                        self.state.current_tab = ViewTab::All;
                    }
                    if ui.selectable_label(self.state.current_tab == ViewTab::Today, "Today").clicked() {
                        self.state.current_tab = ViewTab::Today;
                    }
                    if ui.selectable_label(self.state.current_tab == ViewTab::Upcoming, "Upcoming").clicked() {
                        self.state.current_tab = ViewTab::Upcoming;
                    }
                    if ui.selectable_label(self.state.current_tab == ViewTab::Completed, "Completed").clicked() {
                        self.state.current_tab = ViewTab::Completed;
                    }
                });
                
                ui.add_space(20.0);
                
                // Calendar section (if shown)
                if self.state.show_calendar {
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new("📅")
                                .size(14.0)
                        ));
                        ui.add_space(8.0);
                        ui.add(egui::Label::new(
                            egui::RichText::new("Calendar")
                                .size(14.0)
                                .strong()
                        ));
                    });
                    
                    ui.add_space(10.0);
                    // Clone tasks to resolve borrow checker issue
                    let tasks_clone = self.state.tasks.clone();
                    ui::draw_calendar(ui, &mut self.state, &tasks_clone);
                }
                
                ui.add_space(20.0);
                
                // Save tasks button (more subtle)
                let save_btn_frame = egui::Frame::none()
                    .fill(DARK_PANEL)
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(8.0));
                
                save_btn_frame.show(ui, |ui| {
                    if ui.add(egui::Label::new(
                        egui::RichText::new("💾 Save All Tasks")
                            .color(MUTED_TEXT)
                    ).sense(egui::Sense::click())).clicked() {
                        self.state.save_requested = true;
                    }
                });
            });
        
        // Central panel for task list
        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(DARK_BG)
                .inner_margin(egui::Margin::same(20.0)))
            .show(ctx, |ui| {
                ui::draw_task_list(ui, &mut self.state);
            });
        
        // Add this to trigger an immediate repaint when actions are pending
        if self.state.action.is_some() {
            ctx.request_repaint();
        }
    }
}
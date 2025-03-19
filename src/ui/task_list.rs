use crate::models::{AppState, ViewTab, TaskAction};
use crate::ui::task_item::draw_task_item;
use crate::ui::theme::*;
use crate::Task;
use eframe::egui;

pub fn draw_task_list(ui: &mut egui::Ui, state: &mut AppState) {
    // Section header with icon
    ui.horizontal(|ui| {
        let icon = match state.current_tab {
            ViewTab::All => "📋",
            ViewTab::Today => "📆",
            ViewTab::Upcoming => "⏰",
            ViewTab::Completed => "✅",
        };
        
        let title = match state.current_tab {
            ViewTab::All => "All Tasks",
            ViewTab::Today => "Today's Tasks",
            ViewTab::Upcoming => "Upcoming Tasks",
            ViewTab::Completed => "Completed Tasks",
        };
        
        ui.add(egui::Label::new(
            egui::RichText::new(icon)
                .size(18.0)
        ));
        ui.add_space(8.0);
        ui.add(egui::Label::new(
            egui::RichText::new(title)
                .size(18.0)
                .strong()
        ));
        
        // Display task count
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let completed = state.tasks.iter().filter(|t| t.completed).count();
            let total = state.tasks.len();
            
            ui.label(egui::RichText::new(format!("{}/{} completed", completed, total))
                .color(MUTED_TEXT));
        });
    });
    
    ui.add_space(20.0);
    
    // First, collect task references and compute the indices
    let mut task_data = Vec::new();
    for (original_idx, task) in state.tasks.iter().enumerate() {
        let should_display = match state.current_tab {
            ViewTab::All => true,
            ViewTab::Completed => task.completed,
            _ => !task.completed,
        };
        
        if should_display {
            task_data.push((original_idx, task.clone()));
        }
    }
    
    // Then, use a scrollable area for the task list
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // Display tasks
            if !task_data.is_empty() {
                for (i, (original_idx, task)) in task_data.iter().enumerate() {
                    ui.horizontal(|ui| {
                        // Create a temporary task reference
                        let task_ref = &task;
                        
                        // Handle click events locally
                        let mut clicked_delete = false;
                        let mut clicked_toggle = false;
                        
                        // Draw the task item with local event handling
                        draw_task_item_with_events(
                            ui, 
                            task_ref, 
                            *original_idx, 
                            i % 2 == 0,
                            &mut clicked_delete,
                            &mut clicked_toggle
                        );
                        
                        // Handle events after the UI code
                        if clicked_delete {
                            state.action = Some(TaskAction::Delete(*original_idx));
                        }
                        if clicked_toggle {
                            state.action = Some(TaskAction::ToggleCompletion(*original_idx));
                        }
                    });
                    
                    // Add a separator between items
                    if i < task_data.len() - 1 {
                        ui.add(egui::Separator::default().spacing(4.0));
                    }
                }
            } else {
                // Empty state with icon
                ui.add_space(40.0);
                
                ui.vertical_centered(|ui| {
                    let icon = match state.current_tab {
                        ViewTab::All => "📝",
                        ViewTab::Today => "📆",
                        ViewTab::Upcoming => "⏰",
                        ViewTab::Completed => "✅",
                    };
                    
                    let message = match state.current_tab {
                        ViewTab::All => "No tasks yet",
                        ViewTab::Today => "No tasks due today",
                        ViewTab::Upcoming => "No upcoming tasks",
                        ViewTab::Completed => "No completed tasks",
                    };
                    
                    ui.add(egui::Label::new(
                        egui::RichText::new(icon)
                            .size(32.0)
                    ));
                    ui.add_space(10.0);
                    ui.add(egui::Label::new(
                        egui::RichText::new(message)
                            .size(18.0)
                            .color(MUTED_TEXT)
                    ));
                    ui.add_space(5.0);
                    ui.add(egui::Label::new(
                        egui::RichText::new("Add a task to get started!")
                            .color(MUTED_TEXT)
                    ));
                });
            }
        });
}

// New function that handles events via callback parameters
fn draw_task_item_with_events(
    ui: &mut egui::Ui, 
    task: &Task, 
    index: usize, 
    is_even: bool,
    clicked_delete: &mut bool,
    clicked_toggle: &mut bool
) {
    // Task container
    let task_bg = if is_even { DARKER_BG } else { DARK_BG };
    let task_frame = egui::Frame::none()
        .fill(task_bg)
        .rounding(egui::Rounding::same(8.0))
        .inner_margin(egui::Margin::same(12.0));
    
    task_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            // Checkbox
            let mut completed = task.completed;
            if ui.checkbox(&mut completed, "").changed() {
                *clicked_toggle = true;
            }
            
            // Task description
            let mut text = egui::RichText::new(&task.description)
                .size(16.0);
            
            if task.completed {
                text = text.strikethrough().color(MUTED_TEXT);
            }
            
            ui.label(text);
            
            // Priority indicator
            let priority_color = match task.priority {
                1 => PINK_PRIMARY,
                2 => CYAN_SECONDARY,
                _ => MUTED_TEXT,
            };
            
            let priority_text = match task.priority {
                1 => "High",
                2 => "Medium",
                _ => "Low",
            };
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Delete button
                if ui.button("🗑️").clicked() {
                    println!("Delete button clicked for task {}", index);
                    *clicked_delete = true;
                }
                
                // Show priority
                ui.label(egui::RichText::new(priority_text).color(priority_color));
                
                // Show deadline if set
                if let Some(deadline) = task.deadline {
                    // Convert timestamp to readable date
                    let date = chrono::NaiveDateTime::from_timestamp_opt(deadline, 0).unwrap();
                    ui.label(egui::RichText::new(format!("📅 {}", date.format("%b %d")))
                        .color(MUTED_TEXT));
                }
            });
        });
    });
}
// Updated function signature to handle deletion directly
pub fn handle_delete_task(state: &mut AppState, task_index: usize) {
    // Set the action to be processed
    println!("Setting delete action for task at index {}", task_index);
    state.action = Some(TaskAction::Delete(task_index));
}
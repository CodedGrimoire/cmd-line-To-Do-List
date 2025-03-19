use crate::models::{AppState, TaskAction};
use crate::Task;
use crate::ui::theme::*;
use crate::utils;
use eframe::egui;
use chrono::Utc;
//where the task items are showing as a list
pub fn draw_task_item(
    ui: &mut egui::Ui, 
    task: &Task, 
    index: usize, 
    zebra_stripe: bool,
    state: &mut AppState
) {
    // Get current timestamp for deadline comparison
    let now = Utc::now().timestamp();
    
    // Determine task status for styling
    let is_overdue = task.deadline.map_or(false, |d| d < now && !task.completed);
    let is_approaching = task.deadline.map_or(false, |d| utils::is_approaching_deadline(d));
    
    // Alternate background colors for zebra-striping effect
    let base_color = if zebra_stripe { 
        DARK_PANEL.linear_multiply(1.1)
    } else { 
        DARK_PANEL 
    };
    
    // Border color based on task status
    let border_color = if task.completed {
        GREEN_ACCENT.linear_multiply(0.7)
    } else if is_overdue {
        RED_DEADLINE
    } else if is_approaching {
        ORANGE_ACCENT
    } else {
        priority_color(task.priority)
    };
    
    let task_frame = egui::Frame::none()
        .fill(base_color)
        .rounding(egui::Rounding::same(8.0))
        .stroke(egui::Stroke::new(2.0, border_color))
        .inner_margin(egui::Margin::same(12.0))
        .outer_margin(egui::Margin::same(4.0));
        
    task_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            // Task content container
            ui.vertical(|ui| {
                // Task description with conditional styling
                let text_color = if task.completed { 
                    MUTED_TEXT 
                } else if is_overdue { 
                    RED_DEADLINE 
                } else if is_approaching { 
                    ORANGE_ACCENT 
                } else { 
                    TEXT_COLOR 
                };
                
                let mut text = egui::RichText::new(&task.description)
                    .color(text_color)
                    .size(15.0);
                
                // Apply strikethrough if task is completed
                if task.completed {
                    text = text.strikethrough();
                }
                
                ui.add(egui::Label::new(text).wrap(true));
                
                // Deadline display if present
                if let Some(deadline) = task.deadline {
                    // Format the deadline
                    let deadline_text = format!("Due: {}", utils::format_date_time(deadline));
                    
                    // Determine color based on status
                    let deadline_color = if task.completed {
                        MUTED_TEXT
                    } else if is_overdue {
                        RED_DEADLINE
                    } else if is_approaching {
                        ORANGE_ACCENT
                    } else {
                        MUTED_TEXT
                    };
                    
                    ui.add(egui::Label::new(
                        egui::RichText::new(deadline_text)
                            .color(deadline_color)
                            .size(12.0)
                    ));
                    
                    // Show time remaining/overdue
                    let (status_text, _) = utils::format_time_remaining(deadline, now);
                    
                    ui.add(egui::Label::new(
                        egui::RichText::new(status_text)
                            .color(deadline_color)
                            .size(12.0)
                            .italics()
                    ));
                }
            });
            
            ui.add_space(ui.available_width() * 0.3);
            
            // Action buttons container
            ui.vertical(|ui| {
                // Checkbox for completion status
                let mut completed = task.completed;
                if ui.checkbox(&mut completed, "").changed() && completed != task.completed {
                    state.action = Some(TaskAction::ToggleCompletion(index));
                }
                
                // Delete button with confirmation
                if ui.button("🗑 Delete").clicked() {
                    state.action = Some(TaskAction::Delete(index));
                }
                
                // Calendar button to set/clear deadline
                if ui.button(if task.deadline.is_some() { "📅 Clear Deadline" } else { "➕ Set Deadline" }).clicked() {
                    // Toggle deadline
                    if task.deadline.is_some() {
                        state.action = Some(TaskAction::SetDeadline(index, None));
                    } else if let Some(date) = state.selected_date {
                        // Use selected date for deadline
                        let timestamp = utils::date_to_timestamp(date);
                        state.action = Some(TaskAction::SetDeadline(
                            index, 
                            Some(timestamp)
                        ));
                    }
                }
                
                // Priority indicator
                let priority_color = priority_color(task.priority);
                ui.label(
                    egui::RichText::new(format!("Priority: {}", task.priority))
                        .color(priority_color)
                );
            });
        });
    });
}
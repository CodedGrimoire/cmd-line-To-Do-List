use crate::models::AppState;
use crate::Task;
use crate::ui::theme::*;
use eframe::egui;
use std::time::{Duration, SystemTime};
use chrono::Utc;

// Check for approaching deadlines and create notifications
pub fn check_notifications(state: &mut AppState, tasks: &[Task]) {
    // Only check every 30 seconds
    let now = SystemTime::now();
    if now.duration_since(state.last_notification_check).unwrap_or(Duration::from_secs(0)).as_secs() < 30 {
        return;
    }
    
    state.last_notification_check = now;
    state.notifications.clear();
    
    let current_time = Utc::now().timestamp();
    
    // Check for tasks with deadlines approaching in the next 24 hours
    for task in tasks {
        if task.completed {
            continue;
        }
        
        if let Some(deadline) = task.deadline {
            let time_left = deadline - current_time;
            
            // If deadline is in the future but less than 24 hours away
            if time_left > 0 && time_left < 86400 {
                let hours_left = time_left / 3600;
                state.notifications.push(format!(
                    "Task '{}' is due in {} hours!", 
                    task.description,
                    hours_left
                ));
            }
            // If deadline has passed
            else if time_left < 0 {
                state.notifications.push(format!(
                    "Task '{}' is overdue!", 
                    task.description
                ));
            }
        }
    }
}

// Display notifications panel
pub fn draw_notifications(ui: &mut egui::Ui, notifications: &[String]) {
    if notifications.is_empty() {
        return;
    }
    
    ui.add_space(10.0);
    
    let notification_frame = egui::Frame::none()
        .fill(DARK_PANEL)
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(egui::Margin::same(10.0));
    
    notification_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Label::new(
                egui::RichText::new("🔔")
                    .color(ORANGE_ACCENT)
                    .size(16.0)
            ));
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Notifications")
                .strong());
        });
        
        ui.add_space(5.0);
        
        for notification in notifications {
            ui.label(egui::RichText::new(notification)
                .color(ORANGE_ACCENT));
        }
    });
}
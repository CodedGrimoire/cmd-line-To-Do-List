use crate::models::AppState;
use crate::ui::theme::*;
use crate::utils;
use eframe::egui;

pub fn draw_create_task_panel(ui: &mut egui::Ui, state: &mut AppState) {
    ui.add_space(10.0);
    
    // Section header with icon
    ui.horizontal(|ui| {
        ui.add(egui::Label::new(
            egui::RichText::new("✨")
                .size(18.0)
        ));
        ui.add_space(8.0);
        ui.add(egui::Label::new(
            egui::RichText::new("Create New Task")
                .size(18.0)
                .strong()
        ));
    });
    
    ui.add_space(20.0);
    
    // Task description input with subtle background
    ui.label(egui::RichText::new("Description:").size(15.0));
    ui.add_space(5.0);
    
    let text_edit_frame = egui::Frame::none()
        .fill(DARK_PANEL)
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(egui::Margin::same(8.0));
    
    text_edit_frame.show(ui, |ui| {
        let text_edit = egui::TextEdit::singleline(&mut state.new_task_description)
            .hint_text("What needs to be done?")
            .desired_width(f32::INFINITY);
        ui.add(text_edit);
    });
    
    ui.add_space(18.0);
    
    // Priority selection
    ui.label(egui::RichText::new("Priority:").size(15.0));
    ui.add_space(5.0);
    
    ui.horizontal(|ui| {
        for priority in 1..=5 {
            let is_selected = state.new_task_priority == priority;
            
            // Create priority button with appropriate color
            let priority_color = priority_color(priority);
            
            let btn_frame = egui::Frame::none()
                .fill(if is_selected { priority_color } else { DARK_PANEL })
                .rounding(egui::Rounding::same(6.0))
                .inner_margin(egui::Margin::same(8.0));
            
            btn_frame.show(ui, |ui| {
                if ui.add(egui::Label::new(egui::RichText::new(format!("{}", priority))
                    .color(if is_selected { TEXT_COLOR } else { MUTED_TEXT }))
                    .sense(egui::Sense::click())
                ).clicked() {
                    state.new_task_priority = priority;
                }
            });
            
            ui.add_space(4.0);
        }
    });
    
    ui.add_space(5.0);
    ui.label(egui::RichText::new(format!("Selected: P{}", state.new_task_priority)).color(MUTED_TEXT));
    
    ui.add_space(18.0);
    
    // Deadline selection
    ui.label(egui::RichText::new("Deadline:").size(15.0));
    ui.add_space(5.0);
    
    // Display selected deadline
    let deadline_text = match state.new_task_deadline {
        Some(timestamp) => utils::format_date(timestamp),
        None => "No deadline set".to_string()
    };
    
    let deadline_label = egui::Label::new(
        egui::RichText::new(deadline_text)
            .color(if state.new_task_deadline.is_some() { ORANGE_ACCENT } else { MUTED_TEXT })
    );
    ui.add(deadline_label);
    
    if ui.button("Clear deadline").clicked() {
        state.new_task_deadline = None;
        state.selected_date = None;
    }
    
    ui.add_space(25.0);
    
    // Add task button
    let add_btn_frame = egui::Frame::none()
        .fill(PINK_PRIMARY)
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(egui::Margin::same(8.0));
    
    add_btn_frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(7.0);
            if ui.add(egui::Label::new(
                egui::RichText::new("Add Task")
                    .color(TEXT_COLOR)
                    .size(16.0)
                    .strong()
            ).sense(egui::Sense::click())).clicked() {
                add_task(state);
            }
            ui.add_space(8.0);
        });
    });
}

// Handling for adding a new task
fn add_task(state: &mut AppState) {
    if !state.new_task_description.trim().is_empty() {
        let new_task = crate::Task {
            description: state.new_task_description.clone(),
            completed: false,
            priority: state.new_task_priority,
            deadline: state.new_task_deadline,
        };
        
        state.tasks.push(new_task);
        
        // Reset input
        state.new_task_description.clear();
        
        // Keep deadline and selected date for potential batch task adding
        
        // Save tasks
        state.save_requested = true;
    }
}
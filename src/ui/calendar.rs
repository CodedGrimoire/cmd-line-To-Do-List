use crate::models::AppState;
use crate::ui::theme::*;
use crate::utils;
use chrono::{Local, NaiveDate, Datelike};
use eframe::egui;

pub fn draw_calendar(ui: &mut egui::Ui, state: &mut AppState, tasks: &[crate::Task]) {
    let year = state.calendar_year;
    let month = state.calendar_month;
    
    // Month navigation
    ui.horizontal(|ui| {
        if ui.button("◀").clicked() {
            if state.calendar_month == 1 {
                state.calendar_month = 12;
                state.calendar_year -= 1;
            } else {
                state.calendar_month -= 1;
            }
        }
        
        ui.label(format!("{} {}", MONTHS[(month as usize) - 1], year));
        
        if ui.button("▶").clicked() {
            if state.calendar_month == 12 {
                state.calendar_month = 1;
                state.calendar_year += 1;
            } else {
                state.calendar_month += 1;
            }
        }
        
        // Today button
        if ui.button("Today").clicked() {
            let today = Local::now();
            state.calendar_month = today.month();
            state.calendar_year = today.year();
            state.selected_date = Some(today.date_naive());
        }
    });
    
    ui.add_space(10.0);
    
    // Calendar grid
    let days_in_month = utils::days_in_month(year, month);
    
    // Get first day of month (0 = Monday, 6 = Sunday)
    let first_day = NaiveDate::from_ymd_opt(year, month, 1)
        .unwrap()
        .weekday()
        .num_days_from_monday() as usize;
    
    // Days of week header
    ui.horizontal(|ui| {
        for day in &DAYS_OF_WEEK {
            ui.label(*day);
        }
    });
    
    ui.add_space(5.0);
    
    // Calendar grid
    let mut day = 1;
    let today = Local::now().date_naive();
    
    // Tasks with deadlines in this month
    let tasks_this_month: Vec<&crate::Task> = tasks.iter()
        .filter_map(|task| {
            if let Some(deadline) = task.deadline {
                let date = chrono::NaiveDateTime::from_timestamp_opt(deadline, 0);
                if let Some(date) = date {
                    let date = date.date();
                    if date.year() == year && date.month() == month {
                        return Some(task);
                    }
                }
            }
            None
        })
        .collect();
    
    // Render calendar weeks
    for week in 0..6 {
        if day > days_in_month {
            break;
        }
        
        ui.horizontal(|ui| {
            for weekday in 0..7 {
                if week == 0 && weekday < first_day {
                    // Empty cell before first day of month
                    ui.add_sized([30.0, 30.0], egui::Label::new(""));
                } else if day <= days_in_month {
                    let current_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                    
                    // Check if this date has tasks
                    let has_tasks = tasks_this_month.iter().any(|task| {
                        utils::timestamp_matches_day(
                            task.deadline.unwrap(), 
                            year, 
                            month, 
                            day
                        )
                    });
                    
                    let is_selected = state.selected_date.map_or(false, |d| d == current_date);
                    let is_today = current_date == today;
                    
                    // Day cell styling
                    let day_text = format!("{}", day);
                    let date_btn = egui::Button::new(
                        egui::RichText::new(day_text)
                            .color(if is_today { ORANGE_ACCENT } else { TEXT_COLOR })
                            .strong()
                    )
                    .fill(if is_selected { 
                        PINK_PRIMARY.linear_multiply(0.7) 
                    } else if has_tasks { 
                        DARK_PANEL.linear_multiply(1.2) 
                    } else { 
                        DARK_PANEL 
                    })
                    .min_size(egui::vec2(35.0, 35.0));
                    
                    if ui.add(date_btn).clicked() {
                        if is_selected {
                            // Toggle off if already selected
                            state.selected_date = None;
                        } else {
                            // Select this date for task deadline
                            state.selected_date = Some(current_date);
                            
                            // If adding a new task, set the deadline
                            state.new_task_deadline = Some(utils::date_to_timestamp(current_date));
                        }
                    }
                    
                    day += 1;
                } else {
                    // Empty cell after last day of month
                    ui.add_sized([30.0, 30.0], egui::Label::new(""));
                }
            }
        });
        
        ui.add_space(5.0);
    }
}
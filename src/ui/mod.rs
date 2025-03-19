// src/ui/mod.rs
pub mod theme;
pub mod calendar;
pub mod task_list;
pub mod task_item;
pub mod create_task;
pub mod notifications;

// Re-export UI components
pub use calendar::draw_calendar;
pub use task_list::draw_task_list;
pub use create_task::draw_create_task_panel;
pub use notifications::{check_notifications, draw_notifications};
pub use theme::setup_theme;
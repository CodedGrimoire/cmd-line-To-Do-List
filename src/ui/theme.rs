use eframe::egui;

// Refined color palette
pub const PINK_PRIMARY: egui::Color32 = egui::Color32::from_rgb(152, 98, 175);
// Vibrant pink
pub const PINK_SECONDARY: egui::Color32 = egui::Color32::from_rgb(255, 121, 198); // Softer pink
pub const PINK_ACCENT: egui::Color32 = egui::Color32::from_rgb(189, 147, 249);    // Purple accent
pub const DARK_BG: egui::Color32 = egui::Color32::from_rgb(40, 42, 54);           // Dark background
pub const DARK_PANEL: egui::Color32 = egui::Color32::from_rgb(68, 71, 90);        // Panel background
pub const DARK_ACCENT: egui::Color32 = egui::Color32::from_rgb(98, 114, 164);     // Accent for details
pub const TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(248, 248, 242);     // Bright text
pub const MUTED_TEXT: egui::Color32 = egui::Color32::from_rgb(186, 186, 186);     // Muted text
pub const GREEN_ACCENT: egui::Color32 = egui::Color32::from_rgb(80, 250, 123);    // Green for completed tasks
pub const ORANGE_ACCENT: egui::Color32 = egui::Color32::from_rgb(255, 184, 108);  // Orange for upcoming deadlines
pub const RED_DEADLINE: egui::Color32 = egui::Color32::from_rgb(255, 85, 85);     // Red for overdue tasks
pub const DARKER_BG: egui::Color32 = egui::Color32::from_rgb(32, 34, 46);    // Darker alternate background
pub const CYAN_SECONDARY: egui::Color32 = egui::Color32::from_rgb(139, 233, 253); // Cyan accent color
// Days of week for calendar
pub const DAYS_OF_WEEK: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
pub const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", 
                               "July", "August", "September", "October", "November", "December"];

// Apply theme to context
pub fn setup_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Configure visuals
    style.visuals.panel_fill = DARK_BG;
    style.visuals.window_fill = DARK_BG;
    style.visuals.window_stroke = egui::Stroke::new(1.0, DARK_ACCENT);
    style.visuals.widgets.noninteractive.bg_fill = DARK_BG;
    style.visuals.widgets.inactive.bg_fill = DARK_PANEL;
    style.visuals.widgets.active.bg_fill = PINK_PRIMARY;
    style.visuals.widgets.hovered.bg_fill = PINK_PRIMARY.linear_multiply(0.8);
    
    // Configure text colors
    style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, TEXT_COLOR);
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, TEXT_COLOR);
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, TEXT_COLOR);
    style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, TEXT_COLOR);
    
    // Configure selection
    style.visuals.selection.bg_fill = PINK_PRIMARY.linear_multiply(0.5);
    style.visuals.selection.stroke = egui::Stroke::new(1.0, PINK_PRIMARY);
    
    // Update button rounding
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
    
    ctx.set_style(style);
}

// Priority to color mapping
pub fn priority_color(priority: u8) -> egui::Color32 {
    match priority {
        1 => RED_DEADLINE,
        2 => PINK_PRIMARY,
        3 => PINK_SECONDARY,
        4 => PINK_ACCENT,
        _ => DARK_ACCENT,
    }
}
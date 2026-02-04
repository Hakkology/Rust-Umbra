use egui::{Color32, Context, Visuals};

pub fn apply_theme(ctx: &Context) {
    let mut visuals = Visuals::dark();

    // Backgrounds
    visuals.window_fill = Color32::from_rgb(20, 20, 24); // Very dark blue-ish grey
    visuals.panel_fill = Color32::from_rgb(20, 20, 24);

    // Widgets
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(20, 20, 24);
    visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(200, 200, 220); // Text

    visuals.widgets.inactive.bg_fill = Color32::from_rgb(35, 35, 40); // Slightly lighter
    visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(180, 180, 200);

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(50, 50, 60);
    visuals.widgets.hovered.fg_stroke.color = Color32::WHITE;

    visuals.widgets.active.bg_fill = Color32::from_rgb(70, 70, 90);
    visuals.widgets.active.fg_stroke.color = Color32::WHITE;

    // Selection
    visuals.selection.bg_fill = Color32::from_rgb(60, 100, 180); // Blue accent

    ctx.set_visuals(visuals);
}

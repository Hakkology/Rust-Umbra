//! Reusable UI widgets for the shader editor.
//!
//! These helpers reduce code duplication across the UI.

use egui::Ui;

/// Displays a color picker widget for an RGBA color array.
///
/// Returns `true` if the color was changed.
pub fn color_picker(ui: &mut Ui, rgba: &mut [f32; 4]) -> bool {
    let mut color = egui::Color32::from_rgba_premultiplied(
        (rgba[0] * 255.0) as u8,
        (rgba[1] * 255.0) as u8,
        (rgba[2] * 255.0) as u8,
        (rgba[3] * 255.0) as u8,
    );

    let response = ui.color_edit_button_srgba(&mut color);

    if response.changed() {
        let [r, g, b, a] = color.to_array();
        rgba[0] = r as f32 / 255.0;
        rgba[1] = g as f32 / 255.0;
        rgba[2] = b as f32 / 255.0;
        rgba[3] = a as f32 / 255.0;
        true
    } else {
        false
    }
}

/// Displays a labeled color picker widget.
///
/// Returns `true` if the color was changed.
pub fn labeled_color_picker(ui: &mut Ui, label: &str, rgba: &mut [f32; 4]) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label);
        changed = color_picker(ui, rgba);
    });
    changed
}

/// Displays a Vec4 editor with 4 drag values.
pub fn vec4_editor(ui: &mut Ui, value: &mut [f32; 4]) {
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut value[0]).speed(0.1));
        ui.add(egui::DragValue::new(&mut value[1]).speed(0.1));
        ui.add(egui::DragValue::new(&mut value[2]).speed(0.1));
        ui.add(egui::DragValue::new(&mut value[3]).speed(0.1));
    });
}

/// Displays a Vec2 editor with 2 drag values.
pub fn vec2_editor(ui: &mut Ui, value: &mut [f32; 2]) {
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut value[0]).speed(0.1));
        ui.add(egui::DragValue::new(&mut value[1]).speed(0.1));
    });
}

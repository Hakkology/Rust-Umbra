use crate::ui::framework::AppContext;
use crate::ui::window::{WindowConfig, WindowContent, WindowKind};
use egui::Ui;

pub struct InfoPanel;

impl WindowContent for InfoPanel {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            title: "About Umbra".to_string(),
            kind: WindowKind::Floating,
            default_width: 300.0,
            default_height: 200.0,
            resizable: false,
            collapsible: false,
        }
    }

    fn show(&mut self, ui: &mut Ui, ctx: &mut AppContext) {
        ui.vertical_centered(|ui| {
            ui.heading("Project Umbra");
            ui.label("Shader Node Lab v0.1.0");
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
            ui.label("A tool for visual shader graph editing.");
            ui.label("Built with Rust, wgpu, and egui.");
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("Made by Hakkology")
                    .strong()
                    .color(egui::Color32::from_rgb(200, 150, 50)),
            );
            ui.add_space(20.0);
            if ui.button("Close").clicked() {
                *ctx.close_requested = Some("info".to_string());
            }
        });
    }
}

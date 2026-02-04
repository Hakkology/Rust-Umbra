use crate::common::PropertyValue;
use crate::ui::framework::AppContext;
use egui::Ui;

use crate::ui::window::{WindowConfig, WindowContent, WindowKind};

pub struct PropertiesPanel;

impl WindowContent for PropertiesPanel {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            title: "Properties".to_string(),
            kind: WindowKind::PanelRight,
            default_width: 300.0,
            ..Default::default()
        }
    }

    fn show(&mut self, ui: &mut Ui, app_context: &mut AppContext) {
        self.render_content(ui, app_context);
    }
}

impl PropertiesPanel {
    fn render_content(&self, ui: &mut Ui, app_context: &mut AppContext) {
        let AppContext {
            project,
            generated_shader,
            apply_shader,
            preview_texture_id,
            time: _,
            close_requested: _,
        } = app_context;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Renderer Preview");
            });

            ui.add_space(10.0);

            let width = ui.available_width().max(200.0);
            let height = width;

            ui.image(egui::load::SizedTexture::new(
                *preview_texture_id,
                egui::vec2(width, height),
            ));

            ui.add_space(10.0);
            ui.separator();

            ui.collapsing("Shader Properties", |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Add Float").clicked() {
                        project.add_property("new_float", PropertyValue::Float(0.0));
                    }
                    if ui.button("Add Vec4").clicked() {
                        project.add_property("new_vec4", PropertyValue::Vec4([0.0; 4]));
                    }
                    if ui.button("Add Color").clicked() {
                        project
                            .add_property("new_color", PropertyValue::Color([1.0, 1.0, 1.0, 1.0]));
                    }
                });

                for prop in project.properties.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label(&prop.name);
                        match &mut prop.value {
                            PropertyValue::Float(v) => {
                                ui.add(egui::DragValue::new(v).speed(0.1));
                            }
                            PropertyValue::Color(c) => {
                                crate::ui::widgets::color_picker(ui, c);
                            }
                            PropertyValue::Vec4(v) => {
                                crate::ui::widgets::vec4_editor(ui, v);
                            }
                            PropertyValue::Vec2(v) => {
                                crate::ui::widgets::vec2_editor(ui, v);
                            }
                            _ => {} // Other types not handled
                        }
                    });
                }
            });

            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Generate Shader").clicked() {
                    // We need to call evaluator here.
                    // Since we can't easily import circular dependencies if `evaluator` is in `graph`,
                    // we'll assume `evaluator` is accessible via `crate::graph::eval::Evaluator`.
                    **generated_shader = crate::graph::eval::Evaluator::evaluate(project);
                    **apply_shader = true;
                }
            });

            if !generated_shader.is_empty() {
                ui.add_space(10.0);
                ui.label("Generated WGSL:");
                ui.add(
                    egui::TextEdit::multiline(&mut **generated_shader)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                );
            }
        });
    }
}

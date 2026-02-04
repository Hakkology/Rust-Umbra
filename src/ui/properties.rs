use crate::common::PropertyValue;
use crate::project::UmbraProject;
use egui::Ui;

pub struct PropertiesPanel<'a> {
    pub project: &'a mut UmbraProject,
    pub generated_shader: &'a mut String,
    pub apply_shader: &'a mut bool,
    pub preview_texture_id: egui::TextureId,
}

impl<'a> PropertiesPanel<'a> {
    pub fn show(self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Renderer Preview");
            });

            ui.add_space(10.0);

            let width = ui.available_width().max(200.0);
            let height = width;

            ui.image(egui::load::SizedTexture::new(
                self.preview_texture_id,
                egui::vec2(width, height),
            ));

            ui.add_space(10.0);
            ui.separator();

            ui.collapsing("Shader Properties", |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Add Float").clicked() {
                        self.project
                            .add_property("new_float", PropertyValue::Float(0.0));
                    }
                    if ui.button("Add Vec4").clicked() {
                        self.project
                            .add_property("new_vec4", PropertyValue::Vec4([0.0; 4]));
                    }
                    if ui.button("Add Color").clicked() {
                        self.project
                            .add_property("new_color", PropertyValue::Color([1.0, 1.0, 1.0, 1.0]));
                    }
                });

                for prop in self.project.properties.iter_mut() {
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
                    *self.generated_shader = crate::graph::eval::Evaluator::evaluate(self.project);
                    *self.apply_shader = true;
                }
            });

            if !self.generated_shader.is_empty() {
                ui.add_space(10.0);
                ui.label("Generated WGSL:");
                ui.add(
                    egui::TextEdit::multiline(self.generated_shader)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                );
            }
        });
    }
}

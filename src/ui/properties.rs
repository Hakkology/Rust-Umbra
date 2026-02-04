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
        ui.vertical_centered(|ui| {
            ui.heading("Renderer Preview");
        });

        ui.add_space(10.0);

        // Preview Image using the passed texture ID
        // Note: The caller calculates size logic, or we do it here.
        // Let's rely on the passed size or calculate based on ui width.
        // The previous code did: width = available_width, height = width / aspect.

        // We will just use the passed size or calculate it.
        // If preview_size is zero, we calculate.
        let width = ui.available_width();
        let height = width; // Aspect ratio 1.0 assumed

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
                            let mut color = egui::Color32::from_rgba_premultiplied(
                                (c[0] * 255.0) as u8,
                                (c[1] * 255.0) as u8,
                                (c[2] * 255.0) as u8,
                                (c[3] * 255.0) as u8,
                            );
                            if ui.color_edit_button_srgba(&mut color).changed() {
                                let [r, g, b, a] = color.to_array();
                                c[0] = r as f32 / 255.0;
                                c[1] = g as f32 / 255.0;
                                c[2] = b as f32 / 255.0;
                                c[3] = a as f32 / 255.0;
                            }
                        }
                        PropertyValue::Vec4(v) => {
                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut v[0]).speed(0.1));
                                ui.add(egui::DragValue::new(&mut v[1]).speed(0.1));
                                ui.add(egui::DragValue::new(&mut v[2]).speed(0.1));
                                ui.add(egui::DragValue::new(&mut v[3]).speed(0.1));
                            });
                        }
                        PropertyValue::Vec2(v) => {
                            ui.horizontal(|ui| {
                                ui.add(egui::DragValue::new(&mut v[0]).speed(0.1));
                                ui.add(egui::DragValue::new(&mut v[1]).speed(0.1));
                            });
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(self.generated_shader)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                );
            });
        }
    }
}

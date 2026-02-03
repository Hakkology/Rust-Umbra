use crate::nodes::registry::global_registry;
use crate::nodes::{PropertyValue, register_builtins};
use egui::Ui;
use egui_snarl::{
    InPin, OutPin, Snarl,
    ui::{PinInfo, SnarlViewer},
};
use std::collections::HashMap;

pub mod eval;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum UmbraNode {
    Generic {
        type_name: String,
        properties: HashMap<String, PropertyValue>,
    },
    Float(f32),
    Color(f32, f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Property(String),
    VertexOutput,
    FragmentOutput,
    Position,
}

impl UmbraNode {
    pub fn name(&self) -> String {
        match self {
            UmbraNode::Generic { type_name, .. } => type_name.clone(),
            UmbraNode::Float(_) => "Float".to_string(),
            UmbraNode::Color(_, _, _, _) => "Color".to_string(),
            UmbraNode::Float4(_, _, _, _) => "Float4".to_string(),
            UmbraNode::Property(_) => "Property".to_string(),
            UmbraNode::VertexOutput => "Vertex Output".to_string(),
            UmbraNode::FragmentOutput => "Fragment Output".to_string(),
            UmbraNode::Position => "Position".to_string(),
        }
    }
}

pub struct UmbraViewer;

impl SnarlViewer<UmbraNode> for UmbraViewer {
    fn title(&mut self, node: &UmbraNode) -> String {
        node.name()
    }

    fn inputs(&mut self, node: &UmbraNode) -> usize {
        match node {
            UmbraNode::Generic { type_name, .. } => {
                if let Some(impl_node) = global_registry().read().unwrap().get(type_name) {
                    impl_node.inputs().len()
                } else {
                    0
                }
            }
            UmbraNode::Float(_)
            | UmbraNode::Property(_)
            | UmbraNode::Color(_, _, _, _)
            | UmbraNode::Float4(_, _, _, _)
            | UmbraNode::Position => 0,
            UmbraNode::VertexOutput => 1,
            UmbraNode::FragmentOutput => 1,
        }
    }

    fn outputs(&mut self, node: &UmbraNode) -> usize {
        match node {
            UmbraNode::Generic { type_name, .. } => {
                if let Some(impl_node) = global_registry().read().unwrap().get(type_name) {
                    impl_node.outputs().len()
                } else {
                    0
                }
            }
            UmbraNode::Float(_)
            | UmbraNode::Color(_, _, _, _)
            | UmbraNode::Float4(_, _, _, _)
            | UmbraNode::Property(_)
            | UmbraNode::Position => 1,
            UmbraNode::VertexOutput | UmbraNode::FragmentOutput => 0,
        }
    }

    #[allow(refining_impl_trait)]
    fn show_input(&mut self, pin: &InPin, ui: &mut Ui, snarl: &mut Snarl<UmbraNode>) -> PinInfo {
        let node = &snarl[pin.id.node];
        match node {
            UmbraNode::Generic { type_name, .. } => {
                if let Some(impl_node) = global_registry().read().unwrap().get(type_name) {
                    let inputs = impl_node.inputs();
                    if let Some(input) = inputs.get(pin.id.input) {
                        ui.label(&input.name);
                        // Can customize color based on type_name later
                        PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
                    } else {
                        PinInfo::circle()
                    }
                } else {
                    PinInfo::circle()
                }
            }
            UmbraNode::VertexOutput => {
                ui.label("Position Offset");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(100, 200, 255))
            }
            UmbraNode::FragmentOutput => {
                ui.label("Base Color");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(255, 100, 100))
            }
            _ => PinInfo::circle(),
        }
    }

    #[allow(refining_impl_trait)]
    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, snarl: &mut Snarl<UmbraNode>) -> PinInfo {
        let node = &mut snarl[pin.id.node];
        match node {
            UmbraNode::Generic {
                type_name,
                properties,
            } => {
                if let Some(impl_node) = global_registry().read().unwrap().get(type_name) {
                    let outputs = impl_node.outputs();
                    if let Some(output) = outputs.get(pin.id.output) {
                        ui.label(&output.name);
                        // Show properties if this is the first output, or handle property visualization elsewhere
                        // For now, let's keep it simple.
                        // Maybe show properties in the body? Snarl doesn't have a separate body method yet, usually inputs/outputs cover it.
                        // But we can check if pin.id.output is 0 and render properties there.
                        if pin.id.output == 0 {
                            // Minimal property editor for now
                            // We don't have mutable access to properties here easily if we want to change them based on UI
                            // Wait, `node` is `&mut UmbraNode`, so we do have access!
                            for prop_def in impl_node.define_properties() {
                                if let Some(val) = properties.get_mut(&prop_def.name) {
                                    match val {
                                        PropertyValue::Float(v) => {
                                            ui.horizontal(|ui| {
                                                ui.label(&prop_def.name);
                                                ui.add(egui::DragValue::new(v));
                                            });
                                        }
                                        PropertyValue::Color(r, g, b, a) => {
                                            ui.horizontal(|ui| {
                                                ui.label(&prop_def.name);
                                                let mut color =
                                                    egui::Color32::from_rgba_premultiplied(
                                                        (*r * 255.0) as u8,
                                                        (*g * 255.0) as u8,
                                                        (*b * 255.0) as u8,
                                                        (*a * 255.0) as u8,
                                                    );
                                                if ui.color_edit_button_srgba(&mut color).changed()
                                                {
                                                    let [r_new, g_new, b_new, a_new] =
                                                        color.to_array();
                                                    *r = r_new as f32 / 255.0;
                                                    *g = g_new as f32 / 255.0;
                                                    *b = b_new as f32 / 255.0;
                                                    *a = a_new as f32 / 255.0;
                                                }
                                            });
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
                    } else {
                        PinInfo::circle()
                    }
                } else {
                    PinInfo::circle()
                }
            }
            UmbraNode::Float(val) => {
                ui.add(egui::DragValue::new(val));
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
            }
            UmbraNode::Color(r, g, b, a) => {
                let mut color = egui::Color32::from_rgba_premultiplied(
                    (*r * 255.0) as u8,
                    (*g * 255.0) as u8,
                    (*b * 255.0) as u8,
                    (*a * 255.0) as u8,
                );
                if ui.color_edit_button_srgba(&mut color).changed() {
                    let [r_new, g_new, b_new, a_new] = color.to_array();
                    *r = r_new as f32 / 255.0;
                    *g = g_new as f32 / 255.0;
                    *b = b_new as f32 / 255.0;
                    *a = a_new as f32 / 255.0;
                }
                PinInfo::circle().with_fill(color)
            }
            UmbraNode::Float4(x, y, z, w) => {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(x).speed(0.1));
                    ui.add(egui::DragValue::new(y).speed(0.1));
                    ui.add(egui::DragValue::new(z).speed(0.1));
                    ui.add(egui::DragValue::new(w).speed(0.1));
                });
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 100))
            }
            UmbraNode::Property(name) => {
                ui.label(name.as_str());
                PinInfo::circle().with_fill(egui::Color32::from_rgb(100, 100, 200))
            }
            UmbraNode::Position => {
                ui.label("Mesh Position");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(100, 200, 255))
            }
            _ => {
                ui.label("Out");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
            }
        }
    }

    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl<UmbraNode>) -> bool {
        true
    }

    #[allow(refining_impl_trait)]
    fn show_graph_menu(&mut self, pos: egui::Pos2, ui: &mut Ui, snarl: &mut Snarl<UmbraNode>) {
        ui.label("Add Node");

        let registry = global_registry().read().unwrap();
        let mut categories: HashMap<
            String,
            Vec<std::sync::Arc<dyn crate::nodes::registry::NodeImpl>>,
        > = HashMap::new();

        for node in registry.list() {
            let cat = node.category();
            let cat_name = cat.first().cloned().unwrap_or("Uncategorized".to_string());
            categories.entry(cat_name).or_default().push(node);
        }

        for (category, nodes) in categories {
            ui.menu_button(&category, |ui| {
                for node in nodes {
                    if ui.button(node.name()).clicked() {
                        let mut properties = HashMap::new();
                        for prop in node.define_properties() {
                            properties.insert(prop.name, prop.value);
                        }
                        snarl.insert_node(
                            pos,
                            UmbraNode::Generic {
                                type_name: node.name().to_string(),
                                properties,
                            },
                        );
                        ui.close();
                    }
                }
            });
        }

        ui.separator();

        if ui.button("Float").clicked() {
            snarl.insert_node(pos, UmbraNode::Float(0.0));
            ui.close();
        }
        if ui.button("Float4").clicked() {
            snarl.insert_node(pos, UmbraNode::Float4(0.0, 0.0, 0.0, 0.0));
            ui.close();
        }
        if ui.button("Color").clicked() {
            snarl.insert_node(pos, UmbraNode::Color(1.0, 1.0, 1.0, 1.0));
            ui.close();
        }
        if ui.button("Property").clicked() {
            snarl.insert_node(pos, UmbraNode::Property("unnamed".to_string()));
            ui.close();
        }
        if ui.button("Position").clicked() {
            snarl.insert_node(pos, UmbraNode::Position);
            ui.close();
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GraphEditor {
    pub snarl: Snarl<UmbraNode>,
}

impl GraphEditor {
    pub fn new() -> Self {
        register_builtins();
        let mut snarl = Snarl::new();
        // Add default output nodes
        snarl.insert_node(egui::pos2(400.0, 200.0), UmbraNode::VertexOutput);
        snarl.insert_node(egui::pos2(400.0, 400.0), UmbraNode::FragmentOutput);

        Self { snarl }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, id_source: impl std::hash::Hash) {
        self.snarl.show(
            &mut UmbraViewer,
            &egui_snarl::ui::SnarlStyle::default(),
            id_source,
            ui,
        );
    }
}

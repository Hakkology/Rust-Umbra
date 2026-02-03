use egui::Ui;
use egui_snarl::{
    InPin, OutPin, Snarl,
    ui::{PinInfo, SnarlViewer},
};

pub mod eval;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum UmbraNode {
    Add,
    Multiply,
    Time,
    UV,
    Float(f32),
    Color(f32, f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Property(String),
    VertexOutput,
    FragmentOutput,
    Position,
}

impl UmbraNode {
    pub fn name(&self) -> &str {
        match self {
            UmbraNode::Add => "Add",
            UmbraNode::Multiply => "Multiply",
            UmbraNode::Time => "Time",
            UmbraNode::UV => "UV",
            UmbraNode::Float(_) => "Float",
            UmbraNode::Color(_, _, _, _) => "Color",
            UmbraNode::Float4(_, _, _, _) => "Float4",
            UmbraNode::Property(_) => "Property",
            UmbraNode::VertexOutput => "Vertex Output",
            UmbraNode::FragmentOutput => "Fragment Output",
            UmbraNode::Position => "Position",
        }
    }
}

pub struct UmbraViewer;

impl SnarlViewer<UmbraNode> for UmbraViewer {
    fn title(&mut self, node: &UmbraNode) -> String {
        node.name().to_owned()
    }

    fn inputs(&mut self, node: &UmbraNode) -> usize {
        match node {
            UmbraNode::Add | UmbraNode::Multiply => 2,
            UmbraNode::Time
            | UmbraNode::UV
            | UmbraNode::Float(_)
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
            UmbraNode::Add
            | UmbraNode::Multiply
            | UmbraNode::Time
            | UmbraNode::UV
            | UmbraNode::Float(_)
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
            UmbraNode::Add | UmbraNode::Multiply => {
                let label = if pin.id.input == 0 { "A" } else { "B" };
                ui.label(label);
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
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
            UmbraNode::Time => {
                ui.label("Time");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
            }
            UmbraNode::UV => {
                ui.label("UV");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(100, 200, 100))
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
        if ui.button("Add").clicked() {
            snarl.insert_node(pos, UmbraNode::Add);
            ui.close();
        }
        if ui.button("Multiply").clicked() {
            snarl.insert_node(pos, UmbraNode::Multiply);
            ui.close();
        }
        if ui.button("Time").clicked() {
            snarl.insert_node(pos, UmbraNode::Time);
            ui.close();
        }
        if ui.button("UV").clicked() {
            snarl.insert_node(pos, UmbraNode::UV);
            ui.close();
        }
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

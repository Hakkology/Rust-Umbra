use crate::nodes::PropertyValue;
use egui::Ui;
use egui_snarl::{
    InPin, OutPin, Snarl,
    ui::{PinInfo, SnarlPin, SnarlViewer},
};
use std::borrow::Cow;

pub mod eval;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum UmbraNode {
    Add,
    Multiply,
    Time,
    UV,
    Float(f32),
    Output,
}

impl UmbraNode {
    pub fn name(&self) -> &str {
        match self {
            UmbraNode::Add => "Add",
            UmbraNode::Multiply => "Multiply",
            UmbraNode::Time => "Time",
            UmbraNode::UV => "UV",
            UmbraNode::Float(_) => "Float",
            UmbraNode::Output => "Output",
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
            UmbraNode::Time | UmbraNode::UV | UmbraNode::Float(_) => 0,
            UmbraNode::Output => 1,
        }
    }

    fn outputs(&mut self, node: &UmbraNode) -> usize {
        match node {
            UmbraNode::Add
            | UmbraNode::Multiply
            | UmbraNode::Time
            | UmbraNode::UV
            | UmbraNode::Float(_) => 1,
            UmbraNode::Output => 0,
        }
    }

    fn show_input(&mut self, pin: &InPin, ui: &mut Ui, snarl: &mut Snarl<UmbraNode>) -> PinInfo {
        let node = &snarl[pin.id.node];
        match node {
            UmbraNode::Add | UmbraNode::Multiply => {
                let label = if pin.id.input == 0 { "A" } else { "B" };
                ui.label(label);
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
            }
            UmbraNode::Output => {
                ui.label("Color");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 100, 100))
            }
            _ => PinInfo::circle(),
        }
    }

    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, snarl: &mut Snarl<UmbraNode>) -> PinInfo {
        let node = &mut snarl[pin.id.node];
        match node {
            UmbraNode::Float(val) => {
                ui.add(egui::DragValue::new(val));
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
            }
            UmbraNode::Time => {
                ui.label("Time");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(200, 200, 200))
            }
            UmbraNode::UV => {
                ui.label("UV");
                PinInfo::circle().with_fill(egui::Color32::from_rgb(100, 200, 100))
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
        if ui.button("Output").clicked() {
            snarl.insert_node(pos, UmbraNode::Output);
            ui.close();
        }
    }
}

pub struct GraphEditor {
    pub snarl: Snarl<UmbraNode>,
}

impl GraphEditor {
    pub fn new() -> Self {
        let mut snarl = Snarl::new();
        // Add default output node
        snarl.insert_node(egui::pos2(400.0, 300.0), UmbraNode::Output);

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

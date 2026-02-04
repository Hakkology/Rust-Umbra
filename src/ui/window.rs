use crate::ui::framework::AppContext;

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
pub enum WindowKind {
    Floating,
    PanelRight,
    PanelLeft,
    PanelTop,
    PanelBottom,
}

pub struct WindowConfig {
    pub title: String,
    pub kind: WindowKind,
    pub default_width: f32,
    pub default_height: f32,
    pub resizable: bool,
    pub collapsible: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Window".to_string(),
            kind: WindowKind::Floating,
            default_width: 400.0,
            default_height: 300.0,
            resizable: true,
            collapsible: true,
        }
    }
}

pub trait WindowContent {
    fn config(&self) -> WindowConfig;
    fn show(&mut self, ui: &mut egui::Ui, ctx: &mut AppContext);
}

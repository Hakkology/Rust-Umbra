use crate::ui::window::{WindowContent, WindowKind};
use std::collections::HashMap;

/// Context passed to all views during rendering
pub struct AppContext<'a> {
    pub project: &'a mut crate::file::UmbraProject,
    pub generated_shader: &'a mut String,
    pub apply_shader: &'a mut bool,
    pub preview_texture_id: egui::TextureId,
    #[allow(dead_code)]
    pub time: f32,
    pub close_requested: &'a mut Option<String>,
}

/// Manages UI panels and their visibility states
pub struct UiManager {
    windows: HashMap<String, Box<dyn WindowContent>>,
    open_states: HashMap<String, bool>,
}

impl UiManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            open_states: HashMap::new(),
        }
    }

    pub fn register_view(&mut self, id: &str, content: Box<dyn WindowContent>, default_open: bool) {
        self.windows.insert(id.to_string(), content);
        self.open_states.insert(id.to_string(), default_open);
    }

    pub fn toggle(&mut self, id: &str) {
        if let Some(state) = self.open_states.get_mut(id) {
            *state = !*state;
        }
    }

    pub fn is_open(&self, id: &str) -> bool {
        *self.open_states.get(id).unwrap_or(&false)
    }

    pub fn show(&mut self, ctx: &egui::Context, app_context: &mut AppContext) {
        let window_ids: Vec<String> = self.windows.keys().cloned().collect();

        for id in window_ids {
            let is_open = self.open_states.get(&id).copied().unwrap_or(false);
            if !is_open {
                continue;
            }

            if let Some(content) = self.windows.get_mut(&id) {
                let config = content.config();
                let egui_id = egui::Id::new(id.as_str());

                match config.kind {
                    WindowKind::PanelRight => {
                        egui::SidePanel::right(egui_id)
                            .resizable(config.resizable)
                            .default_width(config.default_width)
                            .show(ctx, |ui| {
                                content.show(ui, app_context);
                            });
                    }
                    WindowKind::Floating => {
                        let mut open = true;
                        egui::Window::new(&config.title)
                            .id(egui_id)
                            .open(&mut open)
                            .resizable(config.resizable)
                            .collapsible(config.collapsible)
                            .default_size([config.default_width, config.default_height])
                            .show(ctx, |ui| {
                                content.show(ui, app_context);
                            });
                        if !open {
                            self.open_states.insert(id.clone(), false);
                        }
                    }
                    _ => {}
                }

                // Handle self-closure if requested by the window content
                if let Some(to_close) = app_context.close_requested.take() {
                    self.open_states.insert(to_close, false);
                }
            }
        }
    }
}

use crate::ui::window::{WindowContent, WindowKind};
use std::collections::HashMap;

/// Context passed to all views during rendering
pub struct AppContext<'a> {
    pub project: &'a mut crate::project::UmbraProject,
    pub generated_shader: &'a mut String,
    pub apply_shader: &'a mut bool,
    pub preview_texture_id: egui::TextureId,
    pub time: f32,
}

/// Helper just for View (optional, if we still need raw View)
/// But we're switching to WindowContent.
/// We will remove View trait definition here if it is no longer used,
/// but let's keep it or alias it if needed. Actually we replace it.

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
        // Collect IDs to avoid borrowing conflicts if necessary,
        // but iterating mutable values is fine.

        let window_ids: Vec<String> = self.windows.keys().cloned().collect();

        for id in window_ids {
            if let Some(open) = self.open_states.get(&id) {
                if !*open {
                    continue;
                }
            } else {
                continue;
            }

            // Temporarily take the content out or borrow it?
            // Borrowing mutably is enough if the closure is executed immediately.
            // But we need to make sure we don't hold 'self' borrow intersecting with closure requirements?
            // Actually, simply getting the mutable ref here:
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
                    WindowKind::PanelLeft => {
                        egui::SidePanel::left(egui_id)
                            .resizable(config.resizable)
                            .default_width(config.default_width)
                            .show(ctx, |ui| {
                                content.show(ui, app_context);
                            });
                    }
                    WindowKind::Floating => {
                        let mut is_open = true; // We checked logical state above. Window open state effectively toggles it? 
                        // Actually egui::Window "open" bool controls the "X" button.
                        // We need to sync it back to our state.
                        if let Some(open) = self.open_states.get(&id) {
                            is_open = *open;
                        }

                        egui::Window::new(&config.title)
                            .id(egui_id)
                            .resizable(config.resizable)
                            .collapsible(config.collapsible)
                            .default_width(config.default_width)
                            .default_height(config.default_height)
                            .open(&mut is_open)
                            .show(ctx, |ui| {
                                content.show(ui, app_context);
                            });

                        // Update state if closed via X
                        if !is_open {
                            if let Some(state) = self.open_states.get_mut(&id) {
                                *state = false;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

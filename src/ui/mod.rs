pub mod framework;
pub mod properties;
pub mod theme;
pub mod widgets;
pub mod window;

// Re-export properties and framework
pub use framework::{AppContext, UiManager};
pub use properties::PropertiesPanel;
pub use window::{WindowConfig, WindowContent, WindowKind};

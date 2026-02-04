pub mod framework;
pub mod info;
pub mod properties;
pub mod theme;
pub mod widgets;
pub mod window;

// Re-export properties and framework
pub use framework::{AppContext, UiManager};
pub use info::InfoPanel;
pub use properties::PropertiesPanel;
#[allow(unused_imports)]
pub use window::{WindowConfig, WindowContent, WindowKind};

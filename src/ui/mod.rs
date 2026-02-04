pub mod properties;
pub mod theme;
// We might not need a dedicated graph_view file if we just use the existing graph module,
// but for consistency let's export it or wrap it here if needed.
// For now, let's keep graph in `crate::graph`.

// Re-export properties
pub use properties::PropertiesPanel;

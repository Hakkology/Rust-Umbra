pub mod input;
#[macro_use]
pub mod macros;
pub mod math;
pub mod registry;

// Re-export common types for convenience
pub use crate::common::{Property, PropertyValue};

pub fn register_builtins() {
    let mut registry = registry::global_registry().write().unwrap();
    registry.register(math::AddNode);
    registry.register(math::MultiplyNode);
    registry.register(input::TimeNode);
    registry.register(input::UVNode);
    registry.register(input::FloatNode);
    registry.register(input::ColorNode);
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod input;
pub mod math;
pub mod registry;

pub fn register_builtins() {
    let mut registry = registry::global_registry().write().unwrap();
    registry.register(math::AddNode);
    registry.register(math::MultiplyNode);
    registry.register(input::TimeNode);
    registry.register(input::UVNode);
    registry.register(input::FloatNode);
    registry.register(input::ColorNode);
    // Add other nodes as I refactor them or create new ones
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum PropertyValue {
    Color(f32, f32, f32, f32),
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Int(i32),
    Bool(bool),
}

impl Default for PropertyValue {
    fn default() -> Self {
        PropertyValue::Float(0.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum ShaderIR {
    Expr(String),
}

#[allow(dead_code)]
pub trait ShaderNode: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self, inputs: &[String], properties: &HashMap<String, PropertyValue>) -> String;
    fn define_properties(&self) -> Vec<Property>;
}

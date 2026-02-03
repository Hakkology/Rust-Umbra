use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod basic;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum PropertyValue {
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

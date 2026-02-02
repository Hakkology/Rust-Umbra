use super::{Property, PropertyValue, ShaderNode};
use std::collections::HashMap;

pub struct AddNode;
impl ShaderNode for AddNode {
    fn name(&self) -> &'static str {
        "Add"
    }
    fn execute(&self, inputs: &[String], _properties: &HashMap<String, PropertyValue>) -> String {
        format!("({} + {})", inputs[0], inputs[1])
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct MultiplyNode;
impl ShaderNode for MultiplyNode {
    fn name(&self) -> &'static str {
        "Multiply"
    }
    fn execute(&self, inputs: &[String], _properties: &HashMap<String, PropertyValue>) -> String {
        format!("({} * {})", inputs[0], inputs[1])
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct TimeNode;
impl ShaderNode for TimeNode {
    fn name(&self) -> &'static str {
        "Time"
    }
    fn execute(&self, _inputs: &[String], _properties: &HashMap<String, PropertyValue>) -> String {
        "uniforms.time".to_string()
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct UVNode;
impl ShaderNode for UVNode {
    fn name(&self) -> &'static str {
        "UV"
    }
    fn execute(&self, _inputs: &[String], _properties: &HashMap<String, PropertyValue>) -> String {
        "in.uv".to_string()
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct FloatNode;
impl ShaderNode for FloatNode {
    fn name(&self) -> &'static str {
        "Float"
    }
    fn execute(&self, _inputs: &[String], properties: &HashMap<String, PropertyValue>) -> String {
        if let Some(PropertyValue::Float(v)) = properties.get("value") {
            format!("{:.3}", v)
        } else {
            "0.0".to_string()
        }
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![Property {
            name: "value".to_string(),
            value: PropertyValue::Float(0.0),
        }]
    }
}

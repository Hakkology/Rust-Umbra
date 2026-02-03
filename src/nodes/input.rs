use super::registry::{InputDefinition, NodeImpl, OutputDefinition};
use super::{Property, PropertyValue};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct TimeNode;
impl NodeImpl for TimeNode {
    fn name(&self) -> &'static str {
        "Time"
    }
    fn category(&self) -> Vec<String> {
        vec!["Input".to_string()]
    }
    fn inputs(&self) -> Vec<InputDefinition> {
        vec![]
    }
    fn outputs(&self) -> Vec<OutputDefinition> {
        vec![OutputDefinition {
            name: "Time".to_string(),
            type_name: "Float".to_string(),
        }]
    }
    fn execute(&self, _inputs: &[String], _properties: &HashMap<String, PropertyValue>) -> String {
        "uniforms.time".to_string()
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![]
    }
}

#[allow(dead_code)]
pub struct UVNode;
impl NodeImpl for UVNode {
    fn name(&self) -> &'static str {
        "UV"
    }
    fn category(&self) -> Vec<String> {
        vec!["Input".to_string()]
    }
    fn inputs(&self) -> Vec<InputDefinition> {
        vec![]
    }
    fn outputs(&self) -> Vec<OutputDefinition> {
        vec![OutputDefinition {
            name: "UV".to_string(),
            type_name: "Vec2".to_string(),
        }]
    }
    fn execute(&self, _inputs: &[String], _properties: &HashMap<String, PropertyValue>) -> String {
        "in.uv".to_string()
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![]
    }
}

#[allow(dead_code)]
pub struct FloatNode;
impl NodeImpl for FloatNode {
    fn name(&self) -> &'static str {
        "Float"
    }
    fn category(&self) -> Vec<String> {
        vec!["Input".to_string()]
    }
    fn inputs(&self) -> Vec<InputDefinition> {
        vec![]
    }
    fn outputs(&self) -> Vec<OutputDefinition> {
        vec![OutputDefinition {
            name: "Out".to_string(),
            type_name: "Float".to_string(),
        }]
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

#[allow(dead_code)]
pub struct ColorNode;
impl NodeImpl for ColorNode {
    fn name(&self) -> &'static str {
        "Color"
    }
    fn category(&self) -> Vec<String> {
        vec!["Input".to_string()]
    }
    fn inputs(&self) -> Vec<InputDefinition> {
        vec![]
    }
    fn outputs(&self) -> Vec<OutputDefinition> {
        vec![OutputDefinition {
            name: "Color".to_string(),
            type_name: "Vec4".to_string(),
        }]
    }
    fn execute(&self, _inputs: &[String], properties: &HashMap<String, PropertyValue>) -> String {
        if let Some(PropertyValue::Color(r, g, b, a)) = properties.get("value") {
            format!("vec4<f32>({:.3}, {:.3}, {:.3}, {:.3})", r, g, b, a)
        } else {
            "vec4<f32>(1.0, 1.0, 1.0, 1.0)".to_string()
        }
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![Property {
            name: "value".to_string(),
            value: PropertyValue::Color(1.0, 1.0, 1.0, 1.0),
        }]
    }
}

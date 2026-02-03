use super::registry::{InputDefinition, NodeImpl, OutputDefinition};
use super::{Property, PropertyValue};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct AddNode;
impl NodeImpl for AddNode {
    fn name(&self) -> &'static str {
        "Add"
    }
    fn category(&self) -> Vec<String> {
        vec!["Math".to_string()]
    }
    fn inputs(&self) -> Vec<InputDefinition> {
        vec![
            InputDefinition {
                name: "A".to_string(),
                type_name: "Float".to_string(),
            },
            InputDefinition {
                name: "B".to_string(),
                type_name: "Float".to_string(),
            },
        ]
    }
    fn outputs(&self) -> Vec<OutputDefinition> {
        vec![OutputDefinition {
            name: "Out".to_string(),
            type_name: "Float".to_string(),
        }]
    }
    fn execute(&self, inputs: &[String], _properties: &HashMap<String, PropertyValue>) -> String {
        format!("({} + {})", inputs[0], inputs[1])
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![]
    }
}

#[allow(dead_code)]
pub struct MultiplyNode;
impl NodeImpl for MultiplyNode {
    fn name(&self) -> &'static str {
        "Multiply"
    }
    fn category(&self) -> Vec<String> {
        vec!["Math".to_string()]
    }
    fn inputs(&self) -> Vec<InputDefinition> {
        vec![
            InputDefinition {
                name: "A".to_string(),
                type_name: "Float".to_string(),
            },
            InputDefinition {
                name: "B".to_string(),
                type_name: "Float".to_string(),
            },
        ]
    }
    fn outputs(&self) -> Vec<OutputDefinition> {
        vec![OutputDefinition {
            name: "Out".to_string(),
            type_name: "Float".to_string(),
        }]
    }
    fn execute(&self, inputs: &[String], _properties: &HashMap<String, PropertyValue>) -> String {
        format!("({} * {})", inputs[0], inputs[1])
    }
    fn define_properties(&self) -> Vec<Property> {
        vec![]
    }
}

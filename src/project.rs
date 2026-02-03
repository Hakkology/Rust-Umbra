use crate::graph::GraphEditor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    Float(f32),
    Color([f32; 4]),
    Vec2([f32; 2]),
    Float4([f32; 4]),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderProperty {
    pub name: String,
    pub value: PropertyValue,
}

#[derive(Serialize, Deserialize)]
pub struct UmbraProject {
    pub name: String,
    pub properties: Vec<ShaderProperty>,
    pub graph: GraphEditor,
}

impl UmbraProject {
    pub fn new() -> Self {
        Self {
            name: "Untitled Project".to_string(),
            properties: Vec::new(),
            graph: GraphEditor::new(),
        }
    }

    pub fn add_property(&mut self, name: &str, value: PropertyValue) {
        self.properties.push(ShaderProperty {
            name: name.to_string(),
            value,
        });
    }
}

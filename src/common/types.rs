use std::path::PathBuf;

/// Unified property value type used across the entire application.
/// This consolidates the previously separate PropertyValue enums.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Color([f32; 4]),
    Int(i32),
    Bool(bool),
    Texture(Option<PathBuf>),
}

impl Default for PropertyValue {
    fn default() -> Self {
        PropertyValue::Float(0.0)
    }
}

impl PropertyValue {
    /// Helper to get float value
    pub fn as_float(&self) -> Option<f32> {
        match self {
            PropertyValue::Float(v) => Some(*v),
            _ => None,
        }
    }

    /// Helper to get color as tuple (r, g, b, a)
    pub fn as_color_tuple(&self) -> Option<(f32, f32, f32, f32)> {
        match self {
            PropertyValue::Color([r, g, b, a]) => Some((*r, *g, *b, *a)),
            PropertyValue::Vec4([r, g, b, a]) => Some((*r, *g, *b, *a)),
            _ => None,
        }
    }
}

/// Property definition for nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
}

/// Shader intermediate representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderIR {
    Expr(String),
}

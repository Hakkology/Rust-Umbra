//! Input nodes for shader graphs
//!
//! These nodes provide input values like time, UV coordinates, and constant values.

use crate::common::PropertyValue;
use crate::define_node;

// ============================================================================
// Time Node - Provides elapsed time uniform
// ============================================================================

define_node!(
    TimeNode,
    name: "Time",
    category: "Input",
    inputs: [],
    outputs: [("Time", "Float")],
    properties: [],
    execute: |_inputs, _properties| {
        "uniforms.time".to_string()
    }
);

// ============================================================================
// UV Node - Provides texture coordinates
// ============================================================================

define_node!(
    UVNode,
    name: "UV",
    category: "Input",
    inputs: [],
    outputs: [("UV", "Vec2")],
    properties: [],
    execute: |_inputs, _properties| {
        "in.uv".to_string()
    }
);

// ============================================================================
// Float Node - Constant float value
// ============================================================================

define_node!(
    FloatNode,
    name: "Float",
    category: "Input",
    inputs: [],
    outputs: [("Out", "Float")],
    properties: [("value", PropertyValue::Float(0.0))],
    execute: |_inputs, properties| {
        if let Some(PropertyValue::Float(v)) = properties.get("value") {
            format!("{:.3}", v)
        } else {
            "0.0".to_string()
        }
    }
);

// ============================================================================
// Color Node - RGBA color value
// ============================================================================

define_node!(
    ColorNode,
    name: "Color",
    category: "Input",
    inputs: [],
    outputs: [("Color", "Vec4")],
    properties: [("value", PropertyValue::Color([1.0, 1.0, 1.0, 1.0]))],
    execute: |_inputs, properties| {
        if let Some(PropertyValue::Color([r, g, b, a])) = properties.get("value") {
            format!("vec4<f32>({:.3}, {:.3}, {:.3}, {:.3})", r, g, b, a)
        } else {
            "vec4<f32>(1.0, 1.0, 1.0, 1.0)".to_string()
        }
    }
);

//! Math nodes for shader graphs
//!
//! These nodes perform mathematical operations on inputs.

// ============================================================================
// Add Node - Adds two values
// ============================================================================

define_node!(
    AddNode,
    name: "Add",
    category: "Math",
    inputs: [("A", "Float"), ("B", "Float")],
    outputs: [("Out", "Float")],
    properties: [],
    execute: |inputs, _properties| {
        format!("({} + {})", inputs[0], inputs[1])
    }
);

// ============================================================================
// Multiply Node - Multiplies two values
// ============================================================================

define_node!(
    MultiplyNode,
    name: "Multiply",
    category: "Math",
    inputs: [("A", "Float"), ("B", "Float")],
    outputs: [("Out", "Float")],
    properties: [],
    execute: |inputs, _properties| {
        format!("({} * {})", inputs[0], inputs[1])
    }
);

// ============================================================================
// Sin Node - Sine function
// ============================================================================

define_node!(
    SinNode,
    name: "Sin",
    category: "Math",
    inputs: [("In", "Float")],
    outputs: [("Out", "Float")],
    properties: [],
    execute: |inputs, _properties| {
        format!("sin({})", inputs[0])
    }
);

// ============================================================================
// Cos Node - Cosine function
// ============================================================================

define_node!(
    CosNode,
    name: "Cos",
    category: "Math",
    inputs: [("In", "Float")],
    outputs: [("Out", "Float")],
    properties: [],
    execute: |inputs, _properties| {
        format!("cos({})", inputs[0])
    }
);

// ============================================================================
// Fract Node - Fractional part
// ============================================================================

define_node!(
    FractNode,
    name: "Fract",
    category: "Math",
    inputs: [("In", "Float")],
    outputs: [("Out", "Float")],
    properties: [],
    execute: |inputs, _properties| {
        format!("fract({})", inputs[0])
    }
);

// ============================================================================
// Abs Node - Absolute value
// ============================================================================

define_node!(
    AbsNode,
    name: "Abs",
    category: "Math",
    inputs: [("In", "Float")],
    outputs: [("Out", "Float")],
    properties: [],
    execute: |inputs, _properties| {
        format!("abs({})", inputs[0])
    }
);

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

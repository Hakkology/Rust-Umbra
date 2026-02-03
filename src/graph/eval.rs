use crate::graph::UmbraNode;
use egui_snarl::{InPinId, NodeId, Snarl};
use std::collections::HashMap;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(snarl: &Snarl<UmbraNode>) -> String {
        // Find the Output node
        let output_node = snarl.node_ids().find(|(_, node)| {
            if let UmbraNode::Output = node {
                true
            } else {
                false
            }
        });

        if let Some((output_node_id, _)) = output_node {
            let mut resolved_vars = HashMap::new();
            let mut code_lines = Vec::new();

            let final_expr =
                Self::evaluate_node(snarl, output_node_id, &mut resolved_vars, &mut code_lines);

            let mut shader = String::new();
            shader.push_str("fn get_color(in: VertexOutput) -> vec4<f32> {\n");
            for line in code_lines {
                shader.push_str(&format!("  {}\n", line));
            }
            shader.push_str(&format!("  return {};\n", final_expr));
            shader.push_str("}\n");
            shader
        } else {
            "fn get_color(in: VertexOutput) -> vec4<f32> { return vec4<f32>(1.0, 0.0, 1.0, 1.0); }"
                .to_string()
        }
    }

    fn evaluate_node(
        snarl: &Snarl<UmbraNode>,
        node_id: NodeId,
        resolved_vars: &mut HashMap<NodeId, String>,
        code_lines: &mut Vec<String>,
    ) -> String {
        if let Some(var_name) = resolved_vars.get(&node_id) {
            return var_name.clone();
        }

        let node = &snarl[node_id];
        let var_name = format!("node_{}", node_id.0);

        let result_expr = match node {
            UmbraNode::Add => {
                let a = Self::evaluate_input(snarl, node_id, 0, resolved_vars, code_lines);
                let b = Self::evaluate_input(snarl, node_id, 1, resolved_vars, code_lines);
                format!("({} + {})", a, b)
            }
            UmbraNode::Multiply => {
                let a = Self::evaluate_input(snarl, node_id, 0, resolved_vars, code_lines);
                let b = Self::evaluate_input(snarl, node_id, 1, resolved_vars, code_lines);
                format!("({} * {})", a, b)
            }
            UmbraNode::Time => "uniforms.time".to_string(),
            UmbraNode::UV => "in.uv".to_string(),
            UmbraNode::Float(val) => {
                format!("{:.3}", val)
            }
            UmbraNode::Output => {
                let color = Self::evaluate_input(snarl, node_id, 0, resolved_vars, code_lines);
                color
            }
        };

        if let UmbraNode::Output = node {
            result_expr
        } else {
            let decl = format!("let {} = {};", var_name, result_expr);
            code_lines.push(decl);
            resolved_vars.insert(node_id, var_name.clone());
            var_name
        }
    }

    fn evaluate_input(
        snarl: &Snarl<UmbraNode>,
        node_id: NodeId,
        input_index: usize,
        resolved_vars: &mut HashMap<NodeId, String>,
        code_lines: &mut Vec<String>,
    ) -> String {
        let in_pin = snarl.in_pin(InPinId {
            node: node_id,
            input: input_index,
        });

        if let Some(remote) = in_pin.remotes.first() {
            Self::evaluate_node(snarl, remote.node, resolved_vars, code_lines)
        } else {
            // Default values if not connected
            match snarl[node_id] {
                UmbraNode::Output => "vec4<f32>(1.0, 1.0, 1.0, 1.0)".to_string(),
                _ => "0.0".to_string(),
            }
        }
    }
}

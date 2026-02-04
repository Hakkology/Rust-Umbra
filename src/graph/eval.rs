use crate::graph::UmbraNode;
use crate::nodes::registry::global_registry;
use crate::project::UmbraProject;
use egui_snarl::{InPinId, NodeId, Snarl};
use std::collections::HashMap;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(project: &UmbraProject) -> String {
        let snarl = &project.graph.snarl;

        let mut shader = String::new();
        // 1. Uniforms
        shader.push_str("struct Uniforms {\n");
        shader.push_str("  view_proj: mat4x4<f32>,\n");
        shader.push_str("  time: f32,\n");
        shader.push_str("  p1: f32,\n");
        shader.push_str("  p2: f32,\n");
        shader.push_str("  p3: f32,\n");
        shader.push_str("  resolution: vec2<f32>,\n");
        shader.push_str("  mouse: vec2<f32>,\n");

        for prop in &project.properties {
            match prop.value {
                crate::common::PropertyValue::Float(_) => {
                    shader.push_str(&format!("  {}: f32,\n", prop.name));
                    shader.push_str(&format!("  _pad_{}: vec3<f32>,\n", prop.name));
                }
                crate::common::PropertyValue::Vec2(_) => {
                    shader.push_str(&format!("  {}: vec2<f32>,\n", prop.name));
                    shader.push_str(&format!("  _pad_{}: vec2<f32>,\n", prop.name));
                }
                crate::common::PropertyValue::Color(_) | crate::common::PropertyValue::Vec4(_) => {
                    shader.push_str(&format!("  {}: vec4<f32>,\n", prop.name));
                }
                _ => {} // Other types not handled for shader generation
            }
        }
        shader.push_str("};\n\n");
        shader.push_str("@group(0) @binding(0) var<uniform> uniforms: Uniforms;\n\n");

        // 2. Vertex Shader
        shader.push_str("struct VertexInput {\n");
        shader.push_str("  @location(0) position: vec3<f32>,\n");
        shader.push_str("  @location(1) normal: vec3<f32>,\n");
        shader.push_str("  @location(2) uv: vec2<f32>,\n");
        shader.push_str("};\n\n");
        shader.push_str("struct VertexOutput {\n");
        shader.push_str("  @builtin(position) clip_position: vec4<f32>,\n");
        shader.push_str("  @location(0) uv: vec2<f32>,\n");
        shader.push_str("  @location(1) world_position: vec3<f32>,\n");
        shader.push_str("};\n\n");

        // Vertex evaluation
        let vs_node = snarl
            .node_ids()
            .find(|(_, n)| matches!(n, UmbraNode::VertexOutput));
        let mut vs_code = Vec::new();
        let vs_expr = if let Some((id, _)) = vs_node {
            let mut vars = HashMap::new();
            Self::evaluate_node(snarl, id, &mut vars, &mut vs_code, true)
        } else {
            "vec3<f32>(0.0)".to_string()
        };

        shader.push_str("@vertex\n");
        shader.push_str("fn vs_main(model: VertexInput) -> VertexOutput {\n");
        shader.push_str("  var out: VertexOutput;\n");
        for line in vs_code {
            shader.push_str(&format!("  {}\n", line));
        }
        shader.push_str(&format!(
            "  out.clip_position = uniforms.view_proj * vec4<f32>(model.position + {}, 1.0);\n",
            vs_expr
        ));
        shader.push_str("  out.uv = model.uv;\n");
        shader.push_str("  out.world_position = model.position;\n");
        shader.push_str("  return out;\n");
        shader.push_str("}\n\n");

        // 3. Fragment Shader
        shader.push_str("@fragment\n");
        shader.push_str("fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {\n");

        let fs_node = snarl
            .node_ids()
            .find(|(_, n)| matches!(n, UmbraNode::FragmentOutput));
        let mut fs_code = Vec::new();
        let fs_expr = if let Some((id, _)) = fs_node {
            let mut vars = HashMap::new();
            Self::evaluate_node(snarl, id, &mut vars, &mut fs_code, false)
        } else {
            "vec4<f32>(1.0, 0.0, 1.0, 1.0)".to_string()
        };

        for line in fs_code {
            shader.push_str(&format!("  {}\n", line));
        }
        shader.push_str(&format!("  return {};\n", fs_expr));
        shader.push_str("}\n");

        shader
    }

    fn evaluate_node(
        snarl: &Snarl<UmbraNode>,
        node_id: NodeId,
        resolved_vars: &mut HashMap<NodeId, String>,
        code_lines: &mut Vec<String>,
        is_vertex: bool,
    ) -> String {
        if let Some(var_name) = resolved_vars.get(&node_id) {
            return var_name.clone();
        }

        let node = &snarl[node_id];
        let var_name = format!("node_{}", node_id.0);

        let result_expr = match node {
            UmbraNode::Generic {
                type_name,
                properties,
            } => {
                if let Some(impl_node) = global_registry().read().unwrap().get(type_name) {
                    let mut inputs = Vec::new();
                    for i in 0..impl_node.inputs().len() {
                        inputs.push(Self::evaluate_input(
                            snarl,
                            node_id,
                            i,
                            resolved_vars,
                            code_lines,
                            is_vertex,
                        ));
                    }
                    impl_node.execute(&inputs, properties)
                } else {
                    format!("/* Unknown Node: {} */ 0.0", type_name)
                }
            }
            UmbraNode::Position => if is_vertex {
                "model.position"
            } else {
                "in.world_position"
            }
            .to_string(),
            UmbraNode::Float(val) => format!("{:.3}", val),
            UmbraNode::Color(r, g, b, a) => {
                format!("vec4<f32>({:.3}, {:.3}, {:.3}, {:.3})", r, g, b, a)
            }
            UmbraNode::Float4(x, y, z, w) => {
                format!("vec4<f32>({:.3}, {:.3}, {:.3}, {:.3})", x, y, z, w)
            }
            UmbraNode::Property(name) => format!("uniforms.{}", name),
            UmbraNode::VertexOutput => {
                Self::evaluate_input(snarl, node_id, 0, resolved_vars, code_lines, is_vertex)
            }
            UmbraNode::FragmentOutput => {
                Self::evaluate_input(snarl, node_id, 0, resolved_vars, code_lines, is_vertex)
            }
        };

        if matches!(node, UmbraNode::VertexOutput | UmbraNode::FragmentOutput) {
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
        is_vertex: bool,
    ) -> String {
        let in_pin = snarl.in_pin(InPinId {
            node: node_id,
            input: input_index,
        });

        if let Some(remote) = in_pin.remotes.first() {
            Self::evaluate_node(snarl, remote.node, resolved_vars, code_lines, is_vertex)
        } else {
            match snarl[node_id] {
                UmbraNode::VertexOutput => "vec3<f32>(0.0)".to_string(),
                UmbraNode::FragmentOutput => "vec4<f32>(1.0, 1.0, 1.0, 1.0)".to_string(),
                _ => "0.0".to_string(),
            }
        }
    }
}

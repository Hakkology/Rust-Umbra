use crate::file::UmbraProject;
use crate::file::export::TargetFormat;
use crate::graph::UmbraNode;
use crate::nodes::registry::global_registry;
use egui_snarl::{InPinId, NodeId, Snarl};
use std::collections::HashMap;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(project: &UmbraProject, format: TargetFormat) -> String {
        match format {
            TargetFormat::Wgsl => Self::evaluate_wgsl(project),
            TargetFormat::Godot => Self::evaluate_godot(project),
            TargetFormat::Generic => Self::evaluate_generic(project),
        }
    }

    fn evaluate_wgsl(project: &UmbraProject) -> String {
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
                _ => {}
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

        let vs_node = snarl
            .node_ids()
            .find(|(_, n)| matches!(n, UmbraNode::VertexOutput));
        let mut vs_code = Vec::new();
        let vs_expr = if let Some((id, _)) = vs_node {
            let mut vars = HashMap::new();
            let mut visit_stack = Vec::new();
            Self::evaluate_node(
                snarl,
                id,
                &mut vars,
                &mut visit_stack,
                &mut vs_code,
                true,
                TargetFormat::Wgsl,
            )
        } else {
            "vec3<f32>(0.0)".to_string()
        };

        shader.push_str("@vertex\n");
        shader.push_str("fn vs_main(model: VertexInput) -> VertexOutput {\n");
        shader.push_str("  var out: VertexOutput;\n");
        for line in vs_code {
            shader.push_str(&format!("  {}\n", line));
        }
        shader.push_str(&format!("  let offset = {};\n", vs_expr));
        shader.push_str("  out.clip_position = uniforms.view_proj * vec4<f32>(model.position + offset.xyz, 1.0);\n");
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
            let mut visit_stack = Vec::new();
            Self::evaluate_node(
                snarl,
                id,
                &mut vars,
                &mut visit_stack,
                &mut fs_code,
                false,
                TargetFormat::Wgsl,
            )
        } else {
            "vec4<f32>(1.0, 0.0, 1.0, 1.0)".to_string()
        };

        for line in fs_code {
            shader.push_str(&format!("  {}\n", line));
        }
        shader.push_str(&format!("  let color_final = {};\n", fs_expr));
        shader.push_str("  return vec4<f32>(color_final.rgba);\n");
        shader.push_str("}\n");

        shader
    }

    fn evaluate_godot(project: &UmbraProject) -> String {
        let snarl = &project.graph.snarl;
        let mut shader = String::new();

        shader.push_str("shader_type spatial;\n\n");
        shader.push_str("uniform float time;\n");

        for prop in &project.properties {
            match prop.value {
                crate::common::PropertyValue::Float(_) => {
                    shader.push_str(&format!("uniform float {};\n", prop.name));
                }
                crate::common::PropertyValue::Vec2(_) => {
                    shader.push_str(&format!("uniform vec2 {};\n", prop.name));
                }
                crate::common::PropertyValue::Color(_) | crate::common::PropertyValue::Vec4(_) => {
                    shader.push_str(&format!("uniform vec4 {} : source_color;\n", prop.name));
                }
                _ => {}
            }
        }
        shader.push_str("\n");

        // Vertex
        let vs_node = snarl
            .node_ids()
            .find(|(_, n)| matches!(n, UmbraNode::VertexOutput));
        let mut vs_code = Vec::new();
        let vs_expr = if let Some((id, _)) = vs_node {
            let mut vars = HashMap::new();
            let mut visit_stack = Vec::new();
            Self::evaluate_node(
                snarl,
                id,
                &mut vars,
                &mut visit_stack,
                &mut vs_code,
                true,
                TargetFormat::Godot,
            )
        } else {
            "vec3(0.0)".to_string()
        };

        shader.push_str("void vertex() {\n");
        for line in vs_code {
            shader.push_str(&format!("  {}\n", line));
        }
        shader.push_str(&format!("  VERTEX += ({}).xyz;\n", vs_expr));
        shader.push_str("}\n\n");

        // Fragment
        let fs_node = snarl
            .node_ids()
            .find(|(_, n)| matches!(n, UmbraNode::FragmentOutput));
        let mut fs_code = Vec::new();
        let fs_expr = if let Some((id, _)) = fs_node {
            let mut vars = HashMap::new();
            let mut visit_stack = Vec::new();
            Self::evaluate_node(
                snarl,
                id,
                &mut vars,
                &mut visit_stack,
                &mut fs_code,
                false,
                TargetFormat::Godot,
            )
        } else {
            "vec4(1.0, 0.0, 1.0, 1.0)".to_string()
        };

        shader.push_str("void fragment() {\n");
        for line in fs_code {
            shader.push_str(&format!("  {}\n", line));
        }
        shader.push_str(&format!("  vec4 color_final = {};\n", fs_expr));
        shader.push_str("  ALBEDO = color_final.rgb;\n");
        shader.push_str("  ALPHA = color_final.a;\n");
        shader.push_str("}\n");

        shader
    }

    fn evaluate_generic(project: &UmbraProject) -> String {
        // Generic .shader (Unity-like or just GLSL)
        // For now, let's treat it as simple GLSL
        let snarl = &project.graph.snarl;
        let mut shader = String::new();

        shader.push_str("// Generic GLSL Shader\n\n");
        shader.push_str("uniform float time;\n");

        for prop in &project.properties {
            match prop.value {
                crate::common::PropertyValue::Float(_) => {
                    shader.push_str(&format!("uniform float {};\n", prop.name));
                }
                crate::common::PropertyValue::Vec2(_) => {
                    shader.push_str(&format!("uniform vec2 {};\n", prop.name));
                }
                crate::common::PropertyValue::Color(_) | crate::common::PropertyValue::Vec4(_) => {
                    shader.push_str(&format!("uniform vec4 {};\n", prop.name));
                }
                _ => {}
            }
        }
        shader.push_str("\n");

        // Simple fragment-only style for generic
        let fs_node = snarl
            .node_ids()
            .find(|(_, n)| matches!(n, UmbraNode::FragmentOutput));
        let mut fs_code = Vec::new();
        let fs_expr = if let Some((id, _)) = fs_node {
            let mut vars = HashMap::new();
            let mut visit_stack = Vec::new();
            Self::evaluate_node(
                snarl,
                id,
                &mut vars,
                &mut visit_stack,
                &mut fs_code,
                false,
                TargetFormat::Generic,
            )
        } else {
            "vec4(1.0, 0.0, 1.0, 1.0)".to_string()
        };

        shader.push_str("void main() {\n");
        for line in fs_code {
            shader.push_str(&format!("  {}\n", line));
        }
        shader.push_str(&format!("  gl_FragColor = {};\n", fs_expr));
        shader.push_str("}\n");

        shader
    }

    fn evaluate_node(
        snarl: &Snarl<UmbraNode>,
        node_id: NodeId,
        resolved_vars: &mut HashMap<NodeId, String>,
        visit_stack: &mut Vec<NodeId>,
        code_lines: &mut Vec<String>,
        is_vertex: bool,
        format: TargetFormat,
    ) -> String {
        if let Some(var_name) = resolved_vars.get(&node_id) {
            return var_name.clone();
        }

        if visit_stack.contains(&node_id) {
            return "0.0".to_string();
        }
        visit_stack.push(node_id);

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
                            visit_stack,
                            code_lines,
                            is_vertex,
                            format,
                        ));
                    }
                    let expr = impl_node.execute(&inputs, properties);
                    // Basic transpile if target is not WGSL
                    if format != TargetFormat::Wgsl {
                        expr.replace("vec3<f32>", "vec3")
                            .replace("vec4<f32>", "vec4")
                            .replace("vec2<f32>", "vec2")
                            .replace("f32", "float")
                    } else {
                        expr
                    }
                } else {
                    "0.0".to_string()
                }
            }
            UmbraNode::Position => {
                match format {
                    TargetFormat::Wgsl => if is_vertex {
                        "model.position"
                    } else {
                        "in.world_position"
                    }
                    .to_string(),
                    TargetFormat::Godot => if is_vertex {
                        "VERTEX"
                    } else {
                        "WORLD_POSITION"
                    }
                    .to_string(), // WORLD_POSITION varies in Godot
                    TargetFormat::Generic => "position".to_string(),
                }
            }
            UmbraNode::Float(val) => format!("{:.3}", val),
            UmbraNode::Color(r, g, b, a) => match format {
                TargetFormat::Wgsl => format!("vec4<f32>({:.3}, {:.3}, {:.3}, {:.3})", r, g, b, a),
                _ => format!("vec4({:.3}, {:.3}, {:.3}, {:.3})", r, g, b, a),
            },
            UmbraNode::Float4(x, y, z, w) => match format {
                TargetFormat::Wgsl => format!("vec4<f32>({:.3}, {:.3}, {:.3}, {:.3})", x, y, z, w),
                _ => format!("vec4({:.3}, {:.3}, {:.3}, {:.3})", x, y, z, w),
            },
            UmbraNode::Property(name) => match format {
                TargetFormat::Wgsl => format!("uniforms.{}", name),
                _ => name.clone(),
            },
            UmbraNode::VertexOutput => Self::evaluate_input(
                snarl,
                node_id,
                0,
                resolved_vars,
                visit_stack,
                code_lines,
                is_vertex,
                format,
            ),
            UmbraNode::FragmentOutput => Self::evaluate_input(
                snarl,
                node_id,
                0,
                resolved_vars,
                visit_stack,
                code_lines,
                is_vertex,
                format,
            ),
        };

        visit_stack.pop();

        if matches!(node, UmbraNode::VertexOutput | UmbraNode::FragmentOutput) {
            result_expr
        } else {
            let decl = match format {
                TargetFormat::Wgsl => format!("let {} = {};", var_name, result_expr),
                _ => {
                    // Primitive type inference for GLSL/GSL is hard with strings,
                    // but since our nodes return typed-looking strings:
                    if result_expr.contains("vec4") {
                        format!("vec4 {} = {};", var_name, result_expr)
                    } else if result_expr.contains("vec3") {
                        format!("vec3 {} = {};", var_name, result_expr)
                    } else if result_expr.contains("vec2") {
                        format!("vec2 {} = {};", var_name, result_expr)
                    } else {
                        format!("float {} = {};", var_name, result_expr)
                    }
                }
            };
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
        visit_stack: &mut Vec<NodeId>,
        code_lines: &mut Vec<String>,
        is_vertex: bool,
        format: TargetFormat,
    ) -> String {
        let in_pin = snarl.in_pin(InPinId {
            node: node_id,
            input: input_index,
        });

        if let Some(remote) = in_pin.remotes.first() {
            Self::evaluate_node(
                snarl,
                remote.node,
                resolved_vars,
                visit_stack,
                code_lines,
                is_vertex,
                format,
            )
        } else {
            match snarl[node_id] {
                UmbraNode::VertexOutput => match format {
                    TargetFormat::Wgsl => "vec3<f32>(0.0)".to_string(),
                    _ => "vec3(0.0)".to_string(),
                },
                UmbraNode::FragmentOutput => match format {
                    TargetFormat::Wgsl => "vec4<f32>(1.0, 1.0, 1.0, 1.0)".to_string(),
                    _ => "vec4(1.0, 1.0, 1.0, 1.0)".to_string(),
                },
                _ => "0.0".to_string(),
            }
        }
    }
}

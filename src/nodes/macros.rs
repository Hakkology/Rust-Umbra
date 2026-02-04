//! Macros for defining shader nodes with less boilerplate.
//!
//! The `define_node!` macro reduces repetitive code when implementing `NodeImpl` trait.

/// Macro to define a shader node with reduced boilerplate.
///
/// # Examples
///
/// ```ignore
/// define_node!(
///     TimeNode,
///     name: "Time",
///     category: "Input",
///     inputs: [],
///     outputs: [("Time", "Float")],
///     properties: [],
///     execute: |_inputs, _properties| {
///         "uniforms.time".to_string()
///     }
/// );
/// ```
#[macro_export]
macro_rules! define_node {
    (
        $node_name:ident,
        name: $display_name:expr,
        category: $category:expr,
        inputs: [$( ($in_name:expr, $in_type:expr) ),* $(,)?],
        outputs: [$( ($out_name:expr, $out_type:expr) ),* $(,)?],
        properties: [$( ($prop_name:expr, $prop_val:expr) ),* $(,)?],
        execute: $exec_fn:expr
    ) => {
        #[allow(dead_code)]
        pub struct $node_name;

        impl $crate::nodes::registry::NodeImpl for $node_name {
            fn name(&self) -> &str {
                $display_name
            }

            fn category(&self) -> Vec<String> {
                vec![$category.to_string()]
            }

            fn inputs(&self) -> Vec<$crate::nodes::registry::InputDefinition> {
                vec![
                    $(
                        $crate::nodes::registry::InputDefinition {
                            name: $in_name.to_string(),
                            type_name: $in_type.to_string(),
                        }
                    ),*
                ]
            }

            fn outputs(&self) -> Vec<$crate::nodes::registry::OutputDefinition> {
                vec![
                    $(
                        $crate::nodes::registry::OutputDefinition {
                            name: $out_name.to_string(),
                            type_name: $out_type.to_string(),
                        }
                    ),*
                ]
            }

            fn define_properties(&self) -> Vec<$crate::common::Property> {
                vec![
                    $(
                        $crate::common::Property {
                            name: $prop_name.to_string(),
                            value: $prop_val,
                        }
                    ),*
                ]
            }

            fn execute(
                &self,
                inputs: &[String],
                properties: &std::collections::HashMap<String, $crate::common::PropertyValue>,
            ) -> String {
                let exec: fn(&[String], &std::collections::HashMap<String, $crate::common::PropertyValue>) -> String = $exec_fn;
                exec(inputs, properties)
            }
        }
    };
}

pub use define_node;

use super::{Property, PropertyValue};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct InputDefinition {
    pub name: String,
    pub type_name: String, // "Float", "Vec3", etc.
}

#[derive(Debug, Clone)]
pub struct OutputDefinition {
    pub name: String,
    pub type_name: String,
}

pub trait NodeImpl: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn category(&self) -> Vec<String>;
    fn inputs(&self) -> Vec<InputDefinition>;
    fn outputs(&self) -> Vec<OutputDefinition>;
    fn execute(&self, inputs: &[String], properties: &HashMap<String, PropertyValue>) -> String;
    fn define_properties(&self) -> Vec<Property>;
}

pub struct NodeRegistry {
    nodes: HashMap<String, Arc<dyn NodeImpl>>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn register(&mut self, node: impl NodeImpl) {
        self.nodes.insert(node.name().to_string(), Arc::new(node));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn NodeImpl>> {
        self.nodes.get(name).cloned()
    }

    pub fn list(&self) -> Vec<Arc<dyn NodeImpl>> {
        let mut list: Vec<_> = self.nodes.values().cloned().collect();
        list.sort_by(|a, b| a.name().cmp(b.name()));
        list
    }
}

// Global registry instance
use std::sync::OnceLock;

pub fn global_registry() -> &'static std::sync::RwLock<NodeRegistry> {
    static REGISTRY: OnceLock<std::sync::RwLock<NodeRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::RwLock::new(NodeRegistry::new()))
}

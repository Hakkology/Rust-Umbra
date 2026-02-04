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

    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let project: Self = serde_json::from_str(&data)?;
        Ok(project)
    }

    pub fn save_as_dialog(&self) -> Option<std::path::PathBuf> {
        let path = rfd::FileDialog::new()
            .add_filter("Umbra Project", &["umbra"])
            .set_file_name(&format!("{}.umbra", self.name))
            .save_file();

        if let Some(path) = &path {
            if let Err(e) = self.save(path) {
                eprintln!("Failed to save project: {}", e);
            }
        }
        path
    }

    pub fn load_dialog() -> Option<Self> {
        let path = rfd::FileDialog::new()
            .add_filter("Umbra Project", &["umbra"])
            .pick_file();

        if let Some(path) = path {
            match Self::load(&path) {
                Ok(project) => return Some(project),
                Err(e) => eprintln!("Failed to load project: {}", e),
            }
        }
        None
    }
}

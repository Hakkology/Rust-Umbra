use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetFormat {
    Wgsl,
    Godot,
    Generic,
}

impl TargetFormat {
    pub fn extension(&self) -> &str {
        match self {
            TargetFormat::Wgsl => "wgsl",
            TargetFormat::Godot => "gdshader",
            TargetFormat::Generic => "shader",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            TargetFormat::Wgsl => "WGSL",
            TargetFormat::Godot => "Godot Shader",
            TargetFormat::Generic => "Generic Shader",
        }
    }
}

pub fn export_shader_dialog(
    shader_code: &str,
    project_name: &str,
    format: TargetFormat,
) -> Option<PathBuf> {
    let ext = format.extension();
    let path = rfd::FileDialog::new()
        .add_filter(format.name(), &[ext])
        .set_file_name(&format!("{}.{}", project_name, ext))
        .save_file();

    if let Some(path) = &path {
        if let Err(e) = fs::write(path, shader_code) {
            eprintln!("Failed to export shader: {}", e);
        }
    }
    path
}

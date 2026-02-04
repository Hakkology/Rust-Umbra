use std::fs;
use std::path::PathBuf;

pub fn save_wgsl_dialog(shader_code: &str, project_name: &str) -> Option<PathBuf> {
    let path = rfd::FileDialog::new()
        .add_filter("WGSL Shader", &["wgsl"])
        .set_file_name(&format!("{}.wgsl", project_name))
        .save_file();

    if let Some(path) = &path {
        if let Err(e) = fs::write(path, shader_code) {
            eprintln!("Failed to export shader: {}", e);
        }
    }
    path
}

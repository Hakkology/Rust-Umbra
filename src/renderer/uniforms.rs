use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub time: f32,
    pub resolution: [f32; 2],
    pub mouse: [f32; 2],
    pub _padding: [f32; 3], // Align to 16 bytes
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
            time: 0.0,
            resolution: [0.0, 0.0],
            mouse: [0.0, 0.0],
            _padding: [0.0; 3],
        }
    }

    pub fn update_view_proj(&mut self, camera: &super::camera::Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

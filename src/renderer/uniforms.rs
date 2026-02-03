use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub time: f32,
    pub p1: f32,
    pub p2: f32,
    pub p3: f32,
    pub resolution: [f32; 2],
    pub mouse: [f32; 2],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
            time: 0.0,
            p1: 0.0,
            p2: 0.0,
            p3: 0.0,
            resolution: [0.0, 0.0],
            mouse: [0.0, 0.0],
        }
    }

    pub fn update_view_proj(&mut self, camera: &super::camera::Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

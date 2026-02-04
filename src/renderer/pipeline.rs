use super::primitives::Vertex;
use super::uniforms::Uniforms;
use wgpu::util::DeviceExt;

pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
}

// ============================================================================
// Helper Functions - Extracted to reduce code duplication
// ============================================================================

/// Creates the uniform bind group layout used by all pipelines
fn create_uniform_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("uniform_bind_group_layout"),
    })
}

/// Creates the uniform bind group
fn create_uniform_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: Some("uniform_bind_group"),
    })
}

/// Standard primitive state for 3D rendering
fn default_primitive_state() -> wgpu::PrimitiveState {
    wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false,
    }
}

/// Standard depth stencil state
fn default_depth_stencil_state() -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: super::Renderer::DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    }
}

/// Standard multisample state
fn default_multisample_state() -> wgpu::MultisampleState {
    wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
    }
}

/// Creates a render pipeline with the given shader and configuration
fn create_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: default_primitive_state(),
        depth_stencil: Some(default_depth_stencil_state()),
        multisample: default_multisample_state(),
        multiview: None,
        cache: None,
    })
}

// ============================================================================
// Pipeline Implementation
// ============================================================================

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader_source: &str,
        mesh: &super::primitives::Mesh,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let uniform_size = std::mem::size_of::<Uniforms>() as wgpu::BufferAddress;
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: uniform_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = create_uniform_bind_group_layout(device);
        let uniform_bind_group =
            create_uniform_bind_group(device, &uniform_bind_group_layout, &uniform_buffer);
        let render_pipeline =
            create_render_pipeline(device, config, &shader, &uniform_bind_group_layout);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: mesh.indices.len() as u32,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn recreate_pipeline(
        &mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader_source: &str,
        uniform_size: u64,
    ) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Recreate uniform buffer if size changed
        if self.uniform_buffer.size() != uniform_size {
            log::info!(
                "Recreating uniform buffer: {} -> {}",
                self.uniform_buffer.size(),
                uniform_size
            );
            self.uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Uniform Buffer (Dynamic)"),
                size: uniform_size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        let uniform_bind_group_layout = create_uniform_bind_group_layout(device);
        self.uniform_bind_group =
            create_uniform_bind_group(device, &uniform_bind_group_layout, &self.uniform_buffer);
        self.render_pipeline =
            create_render_pipeline(device, config, &shader, &uniform_bind_group_layout);
    }
}

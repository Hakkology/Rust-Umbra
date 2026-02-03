pub mod camera;
pub mod gui;
pub mod pipeline;
pub mod primitives;
pub mod uniforms;

use self::camera::{Camera, CameraController};
use self::gui::Gui;
use self::pipeline::Pipeline;
use self::uniforms::Uniforms;
use crate::graph::GraphEditor;
use std::sync::Arc;

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub pipeline: Pipeline,
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub uniforms: Uniforms,
    pub gui: Gui,
    pub graph_editor: GraphEditor,
}

impl Renderer {
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: if size.width == 0 { 1 } else { size.width },
            height: if size.height == 0 { 1 } else { size.height },
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let mesh = primitives::create_uv_sphere(1.0, 32, 32);

        // Initial dummy shader
        let shader_source = r#"
            struct Uniforms {
                view_proj: mat4x4<f32>,
                time: f32,
                resolution: vec2<f32>,
                mouse: vec2<f32>,
            }
            @group(0) @binding(0)
            var<uniform> uniforms: Uniforms;

            struct VertexInput {
                @location(0) position: vec3<f32>,
                @location(1) normal: vec3<f32>,
                @location(2) uv: vec2<f32>,
            }

            struct VertexOutput {
                @builtin(position) clip_position: vec4<f32>,
                @location(0) normal: vec3<f32>,
                @location(1) uv: vec2<f32>,
            }

            @vertex
            fn vs_main(model: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
                out.normal = model.normal;
                out.uv = model.uv;
                return out;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                let color = vec3<f32>(in.normal * 0.5 + 0.5);
                return vec4<f32>(color, 1.0);
            }
        "#;

        let pipeline = Pipeline::new(&device, &config, shader_source, &mesh);

        let camera = Camera {
            eye: (0.0, 1.0, 5.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: glam::Vec3::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0f32.to_radians(),
            znear: 0.1,
            zfar: 100.0,
        };

        let camera_controller = CameraController::new(0.2, 0.01);
        let mut uniforms = Uniforms::new();
        uniforms.resolution = [config.width as f32, config.height as f32];

        let gui = Gui::new(Arc::clone(&window), &device, config.format);
        let graph_editor = GraphEditor::new();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            camera,
            camera_controller,
            uniforms,
            gui,
            graph_editor,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
            self.uniforms.resolution = [new_size.width as f32, new_size.height as f32];
            self.gui.resize(new_size.width, new_size.height);
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.camera_controller.update_camera(&mut self.camera);
        self.uniforms.update_view_proj(&self.camera);
        self.uniforms.time += dt.as_secs_f32();
        self.queue.write_buffer(
            &self.pipeline.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    pub fn handle_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::WindowEvent,
    ) -> bool {
        self.gui.handle_event(window, event)
    }

    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline.render_pipeline);
            render_pass.set_bind_group(0, &self.pipeline.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.pipeline.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.pipeline.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..self.pipeline.num_indices, 0, 0..1);
        }

        // Render GUI
        let graph_editor = &mut self.graph_editor;
        self.gui.render(
            window,
            &self.device,
            &self.queue,
            &mut encoder,
            &view,
            |ctx| {
                egui::SidePanel::right("node_editor")
                    .min_width(400.0)
                    .show(ctx, |ui| {
                        graph_editor.draw(ui, "umbra_node_graph");
                    });
            },
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

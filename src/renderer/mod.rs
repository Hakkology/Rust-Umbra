use crate::project::{PropertyValue, UmbraProject};
use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureView};
use winit::window::Window;

mod camera;
mod gui;
mod pipeline;
mod primitives;
mod uniforms;

use camera::{Camera, CameraController};
use gui::Gui;
use pipeline::Pipeline;
use primitives::create_uv_sphere;
use uniforms::Uniforms;

pub struct Renderer {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub pipeline: Pipeline,
    pub gui: Gui,
    pub project: UmbraProject,
    pub generated_shader: String,
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub uniforms: Uniforms,
    pub depth_texture_view: TextureView,

    // Texture-based preview
    pub preview_view: TextureView,
    pub preview_id: egui::TextureId,
}

impl Renderer {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
    pub const PREVIEW_SIZE: (u32, u32) = (1024, 1024);

    fn create_depth_texture(device: &Device, width: u32, height: u32) -> TextureView {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                ..Default::default()
            })
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
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Preview Texture setup
        let pr_size = wgpu::Extent3d {
            width: Self::PREVIEW_SIZE.0,
            height: Self::PREVIEW_SIZE.1,
            depth_or_array_layers: 1,
        };
        let preview_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Preview Texture"),
            size: pr_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let preview_view = preview_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let _preview_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let depth_texture_view =
            Self::create_depth_texture(&device, Self::PREVIEW_SIZE.0, Self::PREVIEW_SIZE.1);

        let camera = Camera::new(Self::PREVIEW_SIZE.0, Self::PREVIEW_SIZE.1);
        let camera_controller = CameraController::new(4.0, 0.005);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);
        uniforms.resolution = [Self::PREVIEW_SIZE.0 as f32, Self::PREVIEW_SIZE.1 as f32];

        let default_shader = "
            struct Uniforms {
                view_proj: mat4x4<f32>,
                time: f32,
                resolution: vec2<f32>,
                mouse: vec2<f32>,
            };
            @group(0) @binding(0) var<uniform> uniforms: Uniforms;

            struct VertexInput {
                @location(0) position: vec3<f32>,
                @location(1) normal: vec3<f32>,
                @location(2) uv: vec2<f32>,
            };

            struct VertexOutput {
                @builtin(position) clip_position: vec4<f32>,
                @location(0) uv: vec2<f32>,
            };

            @vertex
            fn vs_main(model: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
                out.uv = model.uv;
                return out;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                return vec4<f32>(in.uv, 0.5 + 0.5 * sin(uniforms.time * 2.0), 1.0);
            }
        ";

        let mesh = create_uv_sphere(1.0, 32, 16);
        let pipeline_config = wgpu::SurfaceConfiguration {
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: Self::PREVIEW_SIZE.0,
            height: Self::PREVIEW_SIZE.1,
            ..config.clone()
        };
        let pipeline = Pipeline::new(&device, &pipeline_config, default_shader, &mesh);

        let mut gui = Gui::new(Arc::clone(&window), &device, config.format);
        let preview_id =
            gui.renderer
                .register_native_texture(&device, &preview_view, wgpu::FilterMode::Linear);

        let project = UmbraProject::new();
        let generated_shader = String::new();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            gui,
            project,
            generated_shader,
            camera,
            camera_controller,
            uniforms,
            depth_texture_view,
            preview_view,
            preview_id,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.gui.resize(new_size.width, new_size.height);
        }
    }

    pub fn apply_generated_shader(&mut self) {
        if self.generated_shader.is_empty() {
            return;
        }

        // Calculate uniform size: 16-byte aligned base + properties
        // Base Uniforms size is already 96 bytes (multiple of 16)
        let base_size = 96;
        let mut total_size = base_size;
        for prop in &self.project.properties {
            match prop.value {
                PropertyValue::Float(_) => total_size += 16, // Align each to 16 for safety
                PropertyValue::Vec2(_) => total_size += 16,
                PropertyValue::Color(_) => total_size += 16,
                PropertyValue::Float4(_) => total_size += 16,
            }
        }

        let pipeline_config = wgpu::SurfaceConfiguration {
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: Self::PREVIEW_SIZE.0,
            height: Self::PREVIEW_SIZE.1,
            ..self.config.clone()
        };

        self.pipeline.recreate_pipeline(
            &self.device,
            &pipeline_config,
            &self.generated_shader,
            total_size,
        );
    }

    pub fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
        if self.gui.handle_event(window, event) {
            return true;
        }
        false
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.uniforms.time += dt.as_secs_f32();
        self.camera_controller.update_camera(&mut self.camera);
        self.uniforms.update_view_proj(&self.camera);

        // Build dynamic uniform buffer
        let mut data = Vec::new();
        data.extend_from_slice(bytemuck::cast_slice(&[self.uniforms]));

        // Pad properties to 16-byte alignment
        for prop in &self.project.properties {
            match prop.value {
                PropertyValue::Float(v) => {
                    data.extend_from_slice(bytemuck::bytes_of(&v));
                    data.extend_from_slice(&[0u8; 12]); // Pad to 16
                }
                PropertyValue::Vec2(v) => {
                    data.extend_from_slice(bytemuck::bytes_of(&v));
                    data.extend_from_slice(&[0u8; 8]); // Pad to 16
                }
                PropertyValue::Color(v) => {
                    data.extend_from_slice(bytemuck::bytes_of(&v));
                }
                PropertyValue::Float4(v) => {
                    data.extend_from_slice(bytemuck::bytes_of(&v));
                }
            }
        }

        let buffer_size = self.pipeline.uniform_buffer.size() as usize;
        let write_len = data.len().min(buffer_size);

        self.queue
            .write_buffer(&self.pipeline.uniform_buffer, 0, &data[..write_len]);
    }

    pub fn render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // 1. Render Scene to Preview Texture
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Preview Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.preview_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_viewport(
                0.0,
                0.0,
                Self::PREVIEW_SIZE.0 as f32,
                Self::PREVIEW_SIZE.1 as f32,
                0.0,
                1.0,
            );
            render_pass.set_pipeline(&self.pipeline.render_pipeline);
            render_pass.set_bind_group(0, &self.pipeline.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.pipeline.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.pipeline.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..self.pipeline.num_indices, 0, 0..1);
        }

        // 2. Render GUI
        let project = &mut self.project;
        let generated_shader = &mut self.generated_shader;
        let preview_id = self.preview_id;

        let mut apply_shader = false;

        self.gui.render(
            window,
            &self.device,
            &self.queue,
            &mut encoder,
            &view,
            |ctx| {
                egui::SidePanel::right("renderer_area")
                    .resizable(true)
                    .default_width(self.size.width as f32 * 0.4)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Renderer Preview");
                        });

                        ui.add_space(10.0);

                        // Preview Image
                        let aspect_ratio = 1.0;
                        let width = ui.available_width();
                        let height = width / aspect_ratio;
                        ui.image(egui::load::SizedTexture::new(
                            preview_id,
                            egui::vec2(width, height),
                        ));

                        ui.add_space(10.0);
                        ui.separator();

                        ui.collapsing("Shader Properties", |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Add Float").clicked() {
                                    project.add_property("new_float", PropertyValue::Float(0.0));
                                }
                                if ui.button("Add Float4").clicked() {
                                    project.add_property(
                                        "new_float4",
                                        PropertyValue::Float4([0.0; 4]),
                                    );
                                }
                                if ui.button("Add Color").clicked() {
                                    project.add_property(
                                        "new_color",
                                        PropertyValue::Color([1.0, 1.0, 1.0, 1.0]),
                                    );
                                }
                            });

                            for prop in project.properties.iter_mut() {
                                ui.horizontal(|ui| {
                                    ui.label(&prop.name);
                                    match &mut prop.value {
                                        PropertyValue::Float(v) => {
                                            ui.add(egui::DragValue::new(v).speed(0.1));
                                        }
                                        PropertyValue::Color(c) => {
                                            let mut color = egui::Color32::from_rgba_premultiplied(
                                                (c[0] * 255.0) as u8,
                                                (c[1] * 255.0) as u8,
                                                (c[2] * 255.0) as u8,
                                                (c[3] * 255.0) as u8,
                                            );
                                            if ui.color_edit_button_srgba(&mut color).changed() {
                                                let [r, g, b, a] = color.to_array();
                                                c[0] = r as f32 / 255.0;
                                                c[1] = g as f32 / 255.0;
                                                c[2] = b as f32 / 255.0;
                                                c[3] = a as f32 / 255.0;
                                            }
                                        }
                                        PropertyValue::Float4(v) => {
                                            ui.horizontal(|ui| {
                                                ui.add(egui::DragValue::new(&mut v[0]).speed(0.1));
                                                ui.add(egui::DragValue::new(&mut v[1]).speed(0.1));
                                                ui.add(egui::DragValue::new(&mut v[2]).speed(0.1));
                                                ui.add(egui::DragValue::new(&mut v[3]).speed(0.1));
                                            });
                                        }
                                        PropertyValue::Vec2(v) => {
                                            ui.horizontal(|ui| {
                                                ui.add(egui::DragValue::new(&mut v[0]).speed(0.1));
                                                ui.add(egui::DragValue::new(&mut v[1]).speed(0.1));
                                            });
                                        }
                                    }
                                });
                            }
                        });

                        ui.separator();
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("Generate Shader").clicked() {
                                *generated_shader =
                                    crate::graph::eval::Evaluator::evaluate(project);
                                apply_shader = true;
                            }
                        });

                        if !generated_shader.is_empty() {
                            ui.add_space(10.0);
                            ui.label("Generated WGSL:");
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(generated_shader)
                                        .font(egui::TextStyle::Monospace)
                                        .code_editor()
                                        .lock_focus(true)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        }
                    });

                egui::CentralPanel::default().show(ctx, |ui| {
                    project.graph.draw(ui, "umbra_node_graph");
                });
            },
        );

        if apply_shader {
            self.apply_generated_shader();
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

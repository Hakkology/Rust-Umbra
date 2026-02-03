use egui_wgpu::{Renderer, RendererOptions, ScreenDescriptor};
use egui_winit::State;
use std::sync::Arc;
use wgpu::{Device, TextureFormat, TextureView};
use winit::window::Window;

pub struct Gui {
    pub ctx: egui::Context,
    pub state: State,
    pub renderer: Renderer,
    pub screen_descriptor: ScreenDescriptor,
}

impl Gui {
    pub fn new(window: Arc<Window>, device: &Device, output_format: TextureFormat) -> Self {
        let ctx = egui::Context::default();
        let id = ctx.viewport_id();

        let state = State::new(ctx.clone(), id, &window, None, None, None);
        let renderer = Renderer::new(device, output_format, RendererOptions::default());
        let size = window.inner_size();
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        Self {
            ctx,
            state,
            renderer,
            screen_descriptor,
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
        self.state.on_window_event(window, event).consumed
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    pub fn render(
        &mut self,
        window: &Window,
        device: &Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &TextureView,
        mut run_ui: impl FnMut(&egui::Context),
    ) {
        let raw_input = self.state.take_egui_input(window);
        let full_output = self.ctx.run(raw_input, run_ui);

        self.state
            .handle_platform_output(window, full_output.platform_output);

        let tris = self
            .ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        for (id, image_delta) in full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, id, &image_delta);
        }

        self.renderer
            .update_buffers(device, queue, encoder, &tris, &self.screen_descriptor);

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Egui Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // We need to forget the lifetime here because egui-wgpu expects 'static
            // This is a common pattern in egui-wgpu integration
            self.renderer
                .render(&mut rpass.forget_lifetime(), &tris, &self.screen_descriptor);
        }

        for id in full_output.textures_delta.free {
            self.renderer.free_texture(&id);
        }
    }
}

pub mod common;

mod file;
mod graph;
mod nodes;
mod renderer;
mod ui;

use renderer::Renderer;
use std::sync::Arc;
use std::time::Instant;
use winit::{application::ApplicationHandler, event::*, event_loop::EventLoop, window::Window};

struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    last_render_time: Instant,
    last_mouse_pos: winit::dpi::PhysicalPosition<f64>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            last_render_time: Instant::now(),
            last_mouse_pos: winit::dpi::PhysicalPosition::new(0.0, 0.0),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Project Umbra - Shader Node Lab")
            .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));
        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let renderer = if let Some(r) = self.renderer.as_mut() {
            r
        } else {
            return;
        };

        if renderer.handle_event(self.window.as_ref().unwrap(), &event) {
            return;
        }

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                renderer.camera_controller.is_rotating = state == ElementState::Pressed;
            }
            WindowEvent::CursorMoved { position, .. } => {
                let dx = position.x - self.last_mouse_pos.x;
                let dy = position.y - self.last_mouse_pos.y;
                self.last_mouse_pos = position;

                if renderer.camera_controller.is_rotating {
                    renderer.camera_controller.yaw +=
                        (dx as f32) * renderer.camera_controller.sensitivity;
                    renderer.camera_controller.pitch -=
                        (dy as f32) * renderer.camera_controller.sensitivity;

                    // Constrain pitch
                    let limit = 89.0f32.to_radians();
                    renderer.camera_controller.pitch =
                        renderer.camera_controller.pitch.clamp(-limit, limit);
                }

                renderer.uniforms.mouse = [position.x as f32, position.y as f32];
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        renderer.camera_controller.distance -= y * 0.5;
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        renderer.camera_controller.distance -= pos.y as f32 * 0.01;
                    }
                }
                renderer.camera_controller.distance =
                    renderer.camera_controller.distance.clamp(1.0, 20.0);
            }
            WindowEvent::Resized(physical_size) => {
                renderer.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now - self.last_render_time;
                self.last_render_time = now;

                renderer.update(dt);
                match renderer.render(self.window.as_ref().unwrap()) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}

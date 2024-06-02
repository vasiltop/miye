use crate::renderer;
use crate::state::State;
use winit::event::WindowEvent;
use winit::{application::ApplicationHandler, window::WindowAttributes};

pub enum App {
    Uninitialized,
    Initialized(State),
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();
        let state = State::new(window);

        *self = App::Initialized(state);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let App::Initialized(state) = self {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    state.window.request_redraw();
                    state.update();
                    renderer::draw(state);
                }
                WindowEvent::Resized(size) => {
                    state.surface_config.width = size.width;
                    state.surface_config.height = size.height;
                    state
                        .surface
                        .configure(&state.device, &state.surface_config);
                    state.depth_texture = crate::texture::Texture::create_depth_texture(
                        &state.surface_config,
                        &state.device,
                    );
                    state.camera.reconfigure_aspect_ratio(&state.window);
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    crate::input::handle_keyboard_event(event, state)
                }
                _ => (),
            };
        }
    }
}

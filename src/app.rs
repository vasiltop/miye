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
        *self = App::Initialized(State::new(window));
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
                    renderer::draw(state);
                    state.update();
                }
                _ => (),
            };
        }
    }
}

mod app;
mod renderer;
mod state;

pub fn run() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut app = app::App::Uninitialized;
    event_loop.run_app(&mut app).unwrap();
}

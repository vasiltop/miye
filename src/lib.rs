mod app;
pub mod input;
pub mod instances;
pub mod models;
mod renderer;
mod state;
pub mod texture;

pub fn run() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut app = app::App::Uninitialized;
    event_loop.run_app(&mut app).unwrap();
}

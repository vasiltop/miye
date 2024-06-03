mod app;
pub mod input;
pub mod instances;
pub mod models;
mod renderer;
pub mod state;
pub mod texture;

pub use glam;

pub fn run(f: fn(&mut state::State) -> ()) {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut app = app::App::Uninitialized(f);
    event_loop.run_app(&mut app).unwrap();
}

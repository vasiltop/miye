fn main() {
    miye::run(update);
}

fn update(state: &mut miye::state::State) {
    if state.instances.is_empty() {
        // The textures must be in the same folder as the model
        state.add_instance("./models/cube.obj", miye::glam::Vec3::new(0.0, 0.0, 0.0));
        state.add_instance("./models/cube.obj", miye::glam::Vec3::new(3.0, 0.0, 0.0));
    }
}

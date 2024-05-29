pub mod camera;

#[derive(Debug)]
pub struct Instance {
    pub model: crate::models::Model,
}

impl Instance {
    pub fn new(file_path: &str, state: &crate::state::State) -> Self {
        Self {
            model: crate::models::load_model(file_path, state),
        }
    }
}

#[derive(Debug)]
pub struct Instance {
    pub mesh: Option<obj::Obj>,
}

impl Instance {
    pub fn new(mesh_file_path: Option<&str>) -> Self {
        let mesh = mesh_file_path.map(crate::models::load_model);
        Instance { mesh }
    }
}

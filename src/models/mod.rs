use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

pub fn load_model(file_path: &str) -> Obj {
    let input = BufReader::new(File::open(file_path).unwrap());

    load_obj(input).unwrap()
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl From<&obj::Vertex> for Vertex {
    fn from(value: &obj::Vertex) -> Self {
        Vertex {
            position: value.position,
        }
    }
}

pub fn into_vertex_vec(vec: &Vec<obj::Vertex>) -> Vec<Vertex> {
    vec.iter().map(|v| v.into()).collect()
}

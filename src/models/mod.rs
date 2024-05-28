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
    color: [f32; 3],
}

impl From<&obj::Vertex> for Vertex {
    fn from(value: &obj::Vertex) -> Self {
        Vertex {
            position: value.position,
            color: [1.0, 0.0, 0.0],
        }
    }
}

pub fn into_vertex_vec(vec: &[obj::Vertex]) -> Vec<Vertex> {
    vec.iter().map(|v| v.into()).collect()
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

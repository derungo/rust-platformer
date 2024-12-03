use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
}

impl Vertex {
    pub const fn new(position: [f32; 3], uv: [f32; 2]) -> Self {
        Self { position, uv }
    }

    pub const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

// Define the vertices of a rectangle with UV coordinates
pub const VERTICES: &[Vertex] = &[
    Vertex::new([-0.5, -0.5, 0.0], [0.0, 1.0]), // Bottom-left
    Vertex::new([0.5, -0.5, 0.0], [1.0, 1.0]),  // Bottom-right
    Vertex::new([0.5, 0.5, 0.0], [1.0, 0.0]),   // Top-right
    Vertex::new([-0.5, 0.5, 0.0], [0.0, 0.0]),  // Top-left
];

pub const INDICES: &[u16] = &[
    0, 1, 2, // First triangle
    2, 3, 0, // Second triangle
];

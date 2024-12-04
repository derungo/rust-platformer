// vertex.rs

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        use wgpu::vertex_attr_array;
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = vertex_attr_array![
            0 => Float32x3, // position
            1 => Float32x2  // uv
        ];
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

// Centered vertices
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], uv: [0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], uv: [1.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, 0.0], uv: [1.0, 0.0] },
    Vertex { position: [-0.5,  0.5, 0.0], uv: [0.0, 0.0] },
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

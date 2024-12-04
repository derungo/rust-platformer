// instance.rs
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct InstanceData {
    pub transform: [[f32; 4]; 4], // 64 bytes
    pub sprite_index: f32,        // 4 bytes
    pub _padding1: f32,           // 4 bytes padding
    pub sprite_size: [f32; 2],    // 8 bytes
    pub uv_offset: [f32; 2],      // 8 bytes
    pub uv_scale: [f32; 2],       // 8 bytes
    // Total size: 96 bytes (aligned to 16 bytes)
}

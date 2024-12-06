// renderer.rs

use crate::engine::renderer::vertex::{Vertex, VERTICES, INDICES};

use crate::engine::renderer::texture::{
    create_texture_bind_group, create_texture_bind_group_layout, load_texture, Texture,
};
use crate::engine::renderer::instance::InstanceData;

use wgpu::util::DeviceExt;
use winit::window::Window;


use super::pipeline::create_pipeline;

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub texture_bind_group: wgpu::BindGroup,
    pub tileset_texture: Texture,
    pub tileset_bind_group: wgpu::BindGroup,
    pub tileset_columns: usize,
    pub tileset_rows: usize,
    pub instance_buffer: wgpu::Buffer,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        // Initialize GPU resources
        let instance = wgpu::Instance::default();

        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        // Configure the surface
        let capabilities = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Load the character texture
        let texture = load_texture(&device, &queue, "assets/character/sheets/DinoSprites - tard.png").await;

        // Create texture bind group layout and bind group for the character
        let texture_bind_group_layout = create_texture_bind_group_layout(&device);
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0, // Matches binding(0) in shader
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1, // Matches binding(1) in shader
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("Texture Bind Group"),
        });

        // Load the tileset texture
        let tileset_texture = load_texture(&device, &queue, "assets/tileset/Tileset.png").await;
        let tileset_bind_group =
            create_texture_bind_group(&device, &texture_bind_group_layout, &tileset_texture);

        // Calculate tileset dimensions
        let tile_pixel_size = 16; // Each tile is 16x16 pixels
        let tileset_columns = (tileset_texture.texture.size().width / tile_pixel_size) as usize;
        let tileset_rows = (tileset_texture.texture.size().height / tile_pixel_size) as usize;

        // Create the render pipeline
        let pipeline = create_pipeline(
            &device,
            &config,
            &texture_bind_group_layout,
        );

        // Create vertex and index buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let max_instances = 1000; // Adjust as needed
        let instance_buffer_size = max_instances * std::mem::size_of::<InstanceData>() as wgpu::BufferAddress;

        // Create the instance buffer
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: instance_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            texture_bind_group,
            tileset_texture,
            tileset_bind_group,
            tileset_columns,
            tileset_rows,
            instance_buffer,
        }
    }

    pub fn create_transform_matrix(
        x: f32,
        y: f32,
        scale_x: f32,
        scale_y: f32,
    ) -> [[f32; 4]; 4] {
        [
            [scale_x, 0.0, 0.0, 0.0],
            [0.0, scale_y, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, 0.0, 1.0],
        ]
    }
}


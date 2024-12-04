// renderer.rs

use crate::engine::renderer::{pipeline, vertex};
use crate::engine::renderer::vertex::{Vertex, VERTICES, INDICES};
use crate::engine::renderer::uniform::{
    create_uniform_bind_group, create_uniform_bind_group_layout, create_uniform_buffer, Uniforms,
};
use crate::engine::renderer::texture::{
    create_texture_bind_group, create_texture_bind_group_layout, load_texture, Texture,
};

use wgpu::util::DeviceExt;
use winit::window::Window;

/// The `Renderer` struct handles the rendering pipeline and rendering operations.
pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    texture_bind_group: wgpu::BindGroup,
}

impl Renderer {
    /// Creates a new `Renderer` instance.
    pub async fn new(window: &Window) -> Self {
        // Initialize GPU resources
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(window).unwrap() };
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
            view_formats: vec![capabilities.formats[0]],
        };
        surface.configure(&device, &config);

        // Create uniform bind group layout
        let uniform_bind_group_layout = create_uniform_bind_group_layout(&device);

        // Load the texture
        let texture = load_texture(&device, &queue, "assets/character/sheets/DinoSprites - tard.png").await;

        // Create texture bind group layout and bind group
        let texture_bind_group_layout = create_texture_bind_group_layout(&device);
        let texture_bind_group =
            create_texture_bind_group(&device, &texture_bind_group_layout, &texture);

        // Create the render pipeline
        let pipeline = pipeline::create_pipeline(
            &device,
            &config,
            &uniform_bind_group_layout,
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

        // Initialize uniforms
        let mut uniforms = Uniforms::new();
        uniforms.transform = Self::create_transform_matrix(0.0, 0.0, 1.0, 1.0);
        uniforms.sprite_index = 0.0;
        uniforms.sprite_size = [1.0 / 24.0, 1.0]; // Adjust based on your sprite sheet

        // Create uniform buffer and bind group
        let uniform_buffer = create_uniform_buffer(&device, &uniforms);
        let uniform_bind_group =
            create_uniform_bind_group(&device, &uniform_bind_group_layout, &uniform_buffer);

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            uniform_buffer,
            uniform_bind_group,
            texture_bind_group,
        }
    }

    /// Creates a transformation matrix for rendering.
    pub fn create_transform_matrix(
        x: f32,
        y: f32,
        scale_x: f32,
        scale_y: f32,
    ) -> [[f32; 4]; 4] {
        [
            [scale_x, 0.0,    0.0, 0.0],
            [0.0,    scale_y, 0.0, 0.0],
            [0.0,    0.0,     1.0, 0.0],
            [x,      y,       0.0, 1.0],
        ]
    }
    /// Renders the current frame.
    pub fn render(&self) {
        let output = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Failed to acquire next swap chain texture: {:?}", e);
                return;
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            // Begin render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Clear the screen to a specific color
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            // Set pipeline and bind groups
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_bind_group(1, &self.texture_bind_group, &[]);

            // Set vertex and index buffers
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // Draw the indexed vertices
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        // Submit the commands
        self.queue.submit(Some(encoder.finish()));
        output.present();
    }

    /// Updates the uniform buffer with new transformation and sprite index.
    pub fn update_uniforms(&self, transform: [[f32; 4]; 4], sprite_index: f32) {
        let sprite_size = [1.0 / 24.0, 1.0]; // Adjust based on your sprite sheet

        let uniforms = Uniforms {
            transform,
            sprite_index,
            _padding1: [0; 4],
            sprite_size,
        };

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );
    }
}

use crate::engine::renderer::{pipeline, vertex::INDICES, vertex::VERTICES};
use wgpu::util::DeviceExt;
use winit::window::Window;

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    ground_uniform_buffer: wgpu::Buffer,
    ground_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
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

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }],
            });

        let pipeline = pipeline::create_pipeline(&device, &config, &uniform_bind_group_layout);

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
        let transform_matrix: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0], // scale x
            [0.0, 1.0, 0.0, 0.0], // scale y
            [0.0, 0.0, 1.0, 0.0], // scale z
            [0.0, 0.0, 0.0, 1.0], // position
        ];

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&transform_matrix),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Ground transform
        let ground_transform_matrix: [[f32; 4]; 4] = [
            [2.0, 0.0, 0.0, 0.0], // scale x to fill width
            [0.0, 0.1, 0.0, 0.0], // scale y for ground thickness
            [0.0, 0.0, 1.0, 0.0],
            [0.0, -0.55, 0.0, 1.0], // position at bottom
        ];

        let ground_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Ground Uniform Buffer"),
            contents: bytemuck::cast_slice(&ground_transform_matrix),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let ground_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Ground Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: ground_uniform_buffer.as_entire_binding(),
            }],
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
            uniform_buffer,
            uniform_bind_group,
            ground_uniform_buffer,
            ground_bind_group,
        }
    }

    pub fn update_transform_matrix(&self, x: f32, y: f32, width: f32, height: f32) {
        let transform_matrix: [[f32; 4]; 4] = [
            [width, 0.0, 0.0, 0.0],
            [0.0, height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, 0.0, 1.0],
        ];

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&transform_matrix),
        );
    }

    pub fn render_ground(&self) {
        // No need to update ground transform as it's static
    }

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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
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

            render_pass.set_pipeline(&self.pipeline);

            // Render ground
            render_pass.set_bind_group(0, &self.ground_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

            // Render player
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
    }
}

use crate::engine::renderer::vertex::Vertex;
use crate::engine::renderer::instance::InstanceData;

/// Creates a render pipeline for rendering textured instances.
///
/// This pipeline includes support for:
/// - Vertex attributes for position and texture coordinates.
/// - Instance attributes for transform matrices, sprite indices, and UV offsets.
/// - Alpha blending for semi-transparent textures.
///
/// # Arguments
/// - `device`: The `wgpu::Device` used to create GPU resources.
/// - `config`: The surface configuration that specifies rendering settings like format and size.
/// - `texture_bind_group_layout`: The bind group layout for textures, specifying bindings for texture views and samplers.
///
/// # Returns
/// A `wgpu::RenderPipeline` configured with the specified attributes, shaders, and blending.
///
/// # Notes
/// - The shaders must have entry points named `vs_main` (vertex shader) and `fs_main` (fragment shader).
/// - Ensure the vertex and instance attributes match the shader definitions.
pub fn create_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    // Load the shader module from a WGSL shader file
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
    });

    // Define vertex and instance buffer layouts
    let vertex_layouts = [
        // Layout for vertex attributes
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position attribute (vec3)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Texture coordinate attribute (vec2)
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        },
        // Layout for instance attributes
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // Transform matrix (4x4)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 48,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Sprite index (float)
                wgpu::VertexAttribute {
                    offset: 64,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32,
                },
                // Padding (float)
                wgpu::VertexAttribute {
                    offset: 68,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32,
                },
                // Sprite size (vec2)
                wgpu::VertexAttribute {
                    offset: 72,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // UV offset (vec2)
                wgpu::VertexAttribute {
                    offset: 80,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // UV scale (vec2)
                wgpu::VertexAttribute {
                    offset: 88,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        },
    ];

    // Create the pipeline layout, binding texture resources
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    // Create the render pipeline
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}

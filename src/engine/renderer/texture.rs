//texture.rs
use wgpu::util::DeviceExt;
use image::GenericImageView;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use log::{info, warn};

/// Represents a texture along with its view and sampler.
/// 
/// This structure encapsulates:
/// - The GPU texture object.
/// - A texture view for rendering.
/// - A sampler for filtering and addressing.
pub struct Texture {
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<wgpu::TextureView>,
    pub sampler: Arc<wgpu::Sampler>,
    pub width: u32,  // Add width
    pub height: u32, // Add height
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Self {
            texture: Arc::clone(&self.texture),
            view: Arc::clone(&self.view),
            sampler: Arc::clone(&self.sampler),
            width: self.width,
            height: self.height,
        }
    }
}

/// A global texture cache to avoid reloading the same texture multiple times.
lazy_static::lazy_static! {
    static ref TEXTURE_CACHE: Mutex<HashMap<String, Texture>> = Mutex::new(HashMap::new());
}

/// Loads a texture from a file and creates the associated GPU resources.
/// 
/// # Arguments
/// - `device`: The `wgpu::Device` used to create the GPU resources.
/// - `queue`: The `wgpu::Queue` used to upload texture data to the GPU.
/// - `path`: The file path to the texture image.
/// 
/// # Returns
/// A `Texture` structure containing the loaded texture, its view, and sampler.
/// 
/// # Notes
/// This function uses a global texture cache to avoid redundant loading.
/// If the texture is already cached, it will be returned directly.
pub async fn load_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    path: &str,
) -> Texture {
    let mut cache = TEXTURE_CACHE.lock().unwrap();

    // Check if the texture is already in the cache
    if let Some(texture) = cache.get(path) {
        info!("Using cached texture: {}", path);
        return texture.clone();
    }

    // Load the image using the `image` crate
    info!("Loading texture from file: {}", path);
    let img = image::open(Path::new(path)).expect("Failed to load texture");
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    // Create the GPU texture
    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };
    
    let texture = Arc::new(device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Texture"),
        size,
        mip_level_count: 1, // No mipmaps
        sample_count: 1,    // No multisampling
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb, // sRGB texture format
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    }));

    // Upload pixel data to the GPU texture
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0), // 4 bytes per pixel * width
            rows_per_image: Some(dimensions.1),
        },
        size,
    );

    // Create a texture view and sampler
    let view = Arc::new(texture.create_view(&wgpu::TextureViewDescriptor::default()));
    let sampler = Arc::new(device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Texture Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest, // Nearest-neighbor filtering
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    }));
    let texture = Texture { 
        texture, 
        view, 
        sampler, 
        width: dimensions.0, 
        height: dimensions.1 
    };

    // Cache the texture for future use
    cache.insert(path.to_string(), texture.clone());

    info!("Texture loaded and cached: {}", path);
    texture
}

/// Creates a bind group layout for textures.
/// 
/// This layout specifies two bindings:
/// 1. A 2D texture.
/// 2. A sampler for filtering and addressing modes.
/// 
/// # Arguments
/// - `device`: The `wgpu::Device` used to create the bind group layout.
/// 
/// # Returns
/// A `wgpu::BindGroupLayout` configured for textures and samplers.
pub fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    info!("Creating texture bind group layout");
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
        entries: &[
            // Binding 0: texture
            wgpu::BindGroupLayoutEntry {
                binding: 0, // Matches binding(0) in shader
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
            // Binding 1: sampler
            wgpu::BindGroupLayoutEntry {
                binding: 1, // Matches binding(1) in shader
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

/// Creates a bind group for a specific texture and its sampler.
/// 
/// # Arguments
/// - `device`: The `wgpu::Device` used to create the bind group.
/// - `layout`: The bind group layout created for textures and samplers.
/// - `texture`: The texture to be bound.
/// 
/// # Returns
/// A `wgpu::BindGroup` that binds the texture and sampler.
/// 
/// # Notes
/// Ensure the layout is compatible with the shaders used.
pub fn create_texture_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    texture: &Texture,
) -> wgpu::BindGroup {
    info!("Creating texture bind group");
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture Bind Group"),
        layout,
        entries: &[
            // Texture view binding
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            // Sampler binding
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
    })
}
pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}
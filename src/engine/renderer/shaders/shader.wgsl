// Texture bindings
@group(0) @binding(0)
var sprite_sheet: texture_2d<f32>;
@group(0) @binding(1)
var sprite_sampler: sampler;

// Vertex input and output structures
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,

    // Instance data
    @location(2) transform0: vec4<f32>,
    @location(3) transform1: vec4<f32>,
    @location(4) transform2: vec4<f32>,
    @location(5) transform3: vec4<f32>,
    @location(6) sprite_index: f32,
    @location(7) _padding1: f32,
    @location(8) sprite_size: vec2<f32>,
    @location(9) uv_offset: vec2<f32>,
    @location(10) uv_scale: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) sprite_index: f32,
    @location(2) sprite_size: vec2<f32>,
    @location(3) depth: f32, // Depth for the fragment shader
};

// Vertex shader
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Construct the transformation matrix from instance data
    let transform = mat4x4<f32>(
        input.transform0,
        input.transform1,
        input.transform2,
        input.transform3,
    );

    // Apply transformation
    output.position = transform * vec4<f32>(input.position, 1.0);

    // Assign depth based on layer (e.g., Z-value from input.position)
    output.position.z = input.position.z; // Assign depth to the Z-value
    output.depth = input.position.z;

    // Calculate texture coordinates
    output.tex_coords = input.uv * input.uv_scale + input.uv_offset;

    // Pass through instance data to fragment shader
    output.sprite_index = input.sprite_index;
    output.sprite_size = input.sprite_size;

    return output;
}

// Fragment shader
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Declare adjusted_uv outside the conditional
    var adjusted_uv: vec2<f32>;

    // Decide whether to use sprite logic or UV logic based on sprite_size
    if input.sprite_size.x > 0.0 && input.sprite_size.y > 0.0 {
        // Use sprite logic for character sprites
        let num_sprites_x = 1.0 / input.sprite_size.x;

        let sprite_index_x = input.sprite_index % num_sprites_x;
        let sprite_index_y = floor(input.sprite_index / num_sprites_x);

        let sprite_offset = vec2<f32>(
            sprite_index_x * input.sprite_size.x,
            sprite_index_y * input.sprite_size.y,
        );

        adjusted_uv = sprite_offset + input.tex_coords * input.sprite_size;
    } else {
        // Use UV logic for tiles
        adjusted_uv = input.tex_coords;
    }

    // Sample the texture outside of the conditional using textureSampleLevel
    return textureSampleLevel(sprite_sheet, sprite_sampler, adjusted_uv, 0.0);
}

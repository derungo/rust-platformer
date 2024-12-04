// Uniforms
struct Uniforms {
    transform: mat4x4<f32>,
    sprite_index: f32,
    _padding1: f32,
    sprite_size: vec2<f32>,
};

// Uniform bindings
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

// Texture bindings
@group(1) @binding(0) var sprite_sheet: texture_2d<f32>;
@group(1) @binding(1) var sprite_sampler: sampler;

// Input and output for vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Vertex shader
@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.transform * vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    return output;
}

// Input for fragment shader
struct FragmentInput {
    @location(0) uv: vec2<f32>,
};

// Fragment shader
@fragment
fn fragment_main(input: FragmentInput) -> @location(0) vec4<f32> {
    // Calculate number of sprites in x and y directions
    let num_sprites_x = 1.0 / uniforms.sprite_size.x;
    let num_sprites_y = 1.0 / uniforms.sprite_size.y;

    // Calculate sprite indices
    let sprite_index_x = uniforms.sprite_index % num_sprites_x;
    let sprite_index_y = floor(uniforms.sprite_index / num_sprites_x);

    // Calculate sprite offset
    let sprite_offset = vec2<f32>(
        sprite_index_x * uniforms.sprite_size.x,
        sprite_index_y * uniforms.sprite_size.y
    );

    // Adjust UV coordinates to select the correct sprite
    let adjusted_uv = sprite_offset + input.uv * uniforms.sprite_size;

    // Sample the texture
    return textureSample(sprite_sheet, sprite_sampler, adjusted_uv);
}

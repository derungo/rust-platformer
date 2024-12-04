// Shader Uniforms
struct Uniforms {
     transform: mat4x4<f32>,  // 64 bytes
    sprite_index: f32,       // 4 bytes
    _padding1: f32,          // 4 bytes padding
    sprite_size: vec2<f32>,  // 8 bytes
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
    // Calculate sprite offset based on index
    let sprite_offset = vec2<f32>(
        fract(uniforms.sprite_index / (1.0 / uniforms.sprite_size.x)),
        floor(uniforms.sprite_index * uniforms.sprite_size.y)
    ) * uniforms.sprite_size;

    // Adjust UV coordinates to select the correct sprite
    let adjusted_uv = sprite_offset + input.uv * uniforms.sprite_size;

    // Sample the texture
    return textureSample(sprite_sheet, sprite_sampler, adjusted_uv);
}

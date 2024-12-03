struct Uniforms {
    transform: mat4x4<f32>, // Transform matrix
    sprite_index: f32,      // Index of the sprite (0-23)
    sprite_size: vec2<f32>, // Size of each sprite in UV space
}
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>, // Vertex position
    @location(1) uv: vec2<f32>,       // Vertex UV
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>, // Final vertex position
    @location(0) uv: vec2<f32>,             // UV for the fragment shader
}

@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.transform * vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    return output;
}

@group(1) @binding(0) var sprite_sheet: texture_2d<f32>;
@group(1) @binding(1) var sampler_: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>, // UV passed from vertex shader
}

@fragment
fn fragment_main(input: FragmentInput) -> @location(0) vec4<f32> {
    // Compute sprite offset in UV space
    let sprite_width = uniforms.sprite_size.x;  // Width of a single sprite
    let sprite_offset = vec2<f32>(uniforms.sprite_index * sprite_width, 0.0); // Horizontal offset for the sprite

    // Transform UV coordinates to sample the specific sprite
    let uv = sprite_offset + input.uv * uniforms.sprite_size;

    return textureSample(sprite_sheet, sampler_, uv);
}

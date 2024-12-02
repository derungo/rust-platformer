// Define the output structure for the vertex shader
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) color: vec3<f32>,
};

// Vertex shader
@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.color = color;
    return out;
}

// Fragment shader
@fragment
fn fs_main(
    @location(1) color: vec3<f32>
) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}

struct VertexInput {
    @location(0) vertices: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) vertices: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.vertices = vec4<f32>(input.vertices, 1.0);
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
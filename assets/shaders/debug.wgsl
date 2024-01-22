struct Globals {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexIn {
    @location(0) vertex_pos: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOut {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut{
    var out: VertexOut;
    out.vertex_pos = globals.proj * globals.view * vec4<f32>(in.vertex_pos, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}
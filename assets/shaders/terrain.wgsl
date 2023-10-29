struct Globals {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexInput {
    @builtin(vertex_index) v_index: u32,
    @location(0) vertices: vec3<f32>,
    @location(1) texture_id: u32,
};

struct VertexOutput {
    @builtin(position) vertices: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

const ATLAS_SIZE: u32 = 512u;

fn calculate_uvs(v_index: u32, texture_id: u32) -> vec2<f32> {
    let pixel_x = f32((16u * texture_id) % ATLAS_SIZE);
    let pixel_y = f32((16u * texture_id / ATLAS_SIZE) * 16u);

    switch (v_index % 4u) {
        case 0u: {
            return vec2<f32>(pixel_x / f32(ATLAS_SIZE), (pixel_y + 16.0) / f32(ATLAS_SIZE));
        }
        case 1u: {
            return vec2<f32>((pixel_x + 16.0) / f32(ATLAS_SIZE), (pixel_y + 16.0) / f32(ATLAS_SIZE));
        }
        case 2u: {
            return vec2<f32>((pixel_x + 16.0) / f32(ATLAS_SIZE), pixel_y / f32(ATLAS_SIZE));
        }
        case 3u: {
            return vec2<f32>(pixel_x / f32(ATLAS_SIZE), pixel_y  / f32(ATLAS_SIZE));
        }
        default: {
            return vec2<f32>(0.0, 0.0);
        }
    }
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.vertices = globals.proj * globals.view * vec4<f32>(input.vertices, 1.0);
    output.tex_coords = calculate_uvs(input.v_index, input.texture_id);
    return output;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, input.tex_coords);
}
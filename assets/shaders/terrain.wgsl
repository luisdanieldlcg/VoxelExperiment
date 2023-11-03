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

// TODO: make these configurable
const ATLAS_SIZE: u32 = 512u;
const TEXTURE_WIDTH: u32 = 16u;
const TEXTURE_HEIGHT: u32 = 16u;

fn calculate_uvs(v_index: u32, texture_id: u32) -> vec2<f32> {
    // Recalculate the texture coordinates based on the texture id
    let pixel_x = f32((texture_id % (ATLAS_SIZE / TEXTURE_WIDTH)) * TEXTURE_WIDTH);
    let pixel_y = f32((texture_id / (ATLAS_SIZE / TEXTURE_HEIGHT)) * TEXTURE_HEIGHT);
    
    switch (v_index % 4u) {
          case 0u: {
              return vec2<f32>(pixel_x / f32(ATLAS_SIZE), (pixel_y + f32(TEXTURE_HEIGHT)) / f32(ATLAS_SIZE));
          }
          case 1u: {
              return vec2<f32>((pixel_x + f32(TEXTURE_WIDTH)) / f32(ATLAS_SIZE), (pixel_y + f32(TEXTURE_HEIGHT)) / f32(ATLAS_SIZE));
          }
          case 2u: {
              return vec2<f32>((pixel_x + f32(TEXTURE_WIDTH)) / f32(ATLAS_SIZE), pixel_y / f32(ATLAS_SIZE));
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
    let face_normal = input.vertices[input.v_index];
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

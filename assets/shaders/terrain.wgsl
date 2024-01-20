struct Globals {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    sun_pos: vec3<f32>,
    enable_lighting: u32,
};

@group(0) @binding(0)
var<uniform> globals: Globals;

@group(1) @binding(0)
var<uniform> chunk_pos: vec2<i32>;

struct VertexInput {
    @builtin(vertex_index) v_index: u32,
    @location(0) data: u32,
    @location(1) normal: vec3<i32>,
};

struct VertexOutput {
    @builtin(position) vertices: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<i32>,
    @location(2) local_pos: vec3<f32>,
};

// TODO: make these configurable
const ATLAS_SIZE: u32 = 512u;
const TEXTURE_WIDTH: u32 = 16u;
const TEXTURE_HEIGHT: u32 = 16u;

fn calculate_uvs(v_index: u32, data: u32) -> vec2<f32> {
    // Recalculate the texture coordinates based on the texture id
    // mask 13 bits
    let texture_id = data & 0x1FFFu;
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

fn unpack_vertex_data(data: u32) -> vec3<f32> {
    let x = (data >> 27u) & 0x1Fu;
    let y = (data >> 18u) & 0x1FFu;
    let z = (data >> 13u) & 0x1Fu;
    return vec3<f32>(f32(x), f32(y), f32(z));
}

// fn unpack_vertex_data(data: u32) -> vec3<f32> {
//     let index = (data >> 16u) & 0xFFFFu;
//     let z = index / (16u * 256u);
//     let y = (index - z * 16u * 256u) / 16u;
//     let x = index - z * 16u * 256u - y * 16u;
//     return vec3<f32>(f32(x), f32(y), f32(z));
// }


@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let local_pos = unpack_vertex_data(input.data);
    let world_pos = vec3<f32>(
        f32(chunk_pos.x) * 16.0 + local_pos.x,
        local_pos.y,
        f32(chunk_pos.y) * 16.0 + local_pos.z
    );
    output.vertices = globals.proj * globals.view * vec4<f32>(world_pos, 1.0);
    output.tex_coords = calculate_uvs(input.v_index, input.data);
    output.normal = input.normal;
    output.local_pos = local_pos;
    return output;
}

@group(0) @binding(1)
var texture: texture_2d<f32>;
@group(0) @binding(2)
var texture_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let obj_color = textureSample(texture, texture_sampler, input.tex_coords);
    if (globals.enable_lighting == 0u) {
        return obj_color;
    }
    let ambient_factor = 0.36;
    let light_color = vec3<f32>(1.0, 1.0, 1.0);
    let ambient = ambient_factor * light_color;
    let light_dir = normalize(globals.sun_pos - input.local_pos);
    let diff = max(dot(vec3<f32>(input.normal), light_dir), 0.0);
    let diffuse = diff * light_color;
    let result = (diffuse + ambient) * obj_color.xyz;
    return vec4<f32>(result, obj_color.w);
}

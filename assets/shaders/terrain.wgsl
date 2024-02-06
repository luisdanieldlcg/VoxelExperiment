struct Uniforms {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    atlas_size: u32,
    atlas_tile_size: u32,
    sun_dir: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexIn {
    @location(0) vertex_pos: vec3<f32>,
    @location(1) texture_id: u32,
    @location(2) normal: vec3<f32>,
}

struct VertexOut {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) normal: vec3<f32>,
}

fn calculate_texture_coordinates(v_index: u32, texture_id: u32) -> vec2<f32> {
    let tile_width = uniforms.atlas_tile_size;
    let tile_height = uniforms.atlas_tile_size;
    let tiles_per_row = uniforms.atlas_size / tile_width; 
    let pixel_x = f32((texture_id % tiles_per_row) * tile_width);
    let pixel_y = f32((texture_id / tiles_per_row) * tile_height);

    switch (v_index % 4u) {
          case 0u: {
              return vec2<f32>(pixel_x / f32(uniforms.atlas_size), (pixel_y + f32(tile_height)) / f32(uniforms.atlas_size));
          }
          case 1u: {
              return vec2<f32>((pixel_x + f32(tile_width)) / f32(uniforms.atlas_size), (pixel_y + f32(tile_height)) / f32(uniforms.atlas_size));
          }
          case 2u: {
              return vec2<f32>((pixel_x + f32(tile_width)) / f32(uniforms.atlas_size), pixel_y / f32(uniforms.atlas_size));
          }
          case 3u: {
              return vec2<f32>(pixel_x / f32(uniforms.atlas_size), pixel_y  / f32(uniforms.atlas_size));
          }
          default: {
              return vec2<f32>(0.0, 0.0);
          }
      }
}


@vertex
fn vs_main(in: VertexIn, @builtin(vertex_index) v_index: u32) -> VertexOut{
    var out: VertexOut;
    out.vertex_pos = uniforms.proj * uniforms.view * vec4<f32>(in.vertex_pos, 1.0);
    out.tex_coord = calculate_texture_coordinates(v_index, in.texture_id);
    out.normal = in.normal;
    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let sun_pos = vec3<f32>(0.0, 1.0, -2.0);
    let ambient_color = vec4<f32>(0.7, 0.7, 0.7, 1.0);
    let light_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    let lit_factor = max(dot(in.normal, sun_pos), 0.0);
    let texture_sample = textureSample(texture, texture_sampler, in.tex_coord);
    let dark_shade_factor = light_color * lit_factor;
    let final_color = texture_sample * (ambient_color + dark_shade_factor);
    return final_color;
}

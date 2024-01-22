use vek::{LineSegment3, Vec3};

use crate::render::pipelines::debug::DebugVertex;


// /// ```text  
// ///       h-----g
// ///      /|    /|
// ///     / |   / |
// ///    /  |  /  |
// ///   /   e-/-- f
// ///  /   / /   /
// /// d-----c   /
// /// |  /  |  /
// /// | /   | /
// /// |/    |/
// /// a-----b
// /// ```
// pub fn box_wireframe_mesh(
//     center: Vec3<f32>,
//     color: [f32; 4],
//     size: Vec3<f32>,
// ) -> Mesh<DebugVertex> {
//     // The idea is to draw a box around the `pos` using line meshes.

//     let (x, y, z) = center.into_tuple();
//     let (dist_x, dist_y, dist_z) = size.map(|x| x / 2.0).into_tuple();
//     let mut mesh = Mesh::new();

//     let ab = LineSegment3 {
//         start: Vec3::new(x - dist_x, y - dist_y, z - dist_z),
//         end: Vec3::new(x + dist_x, y - dist_y, z - dist_z),
//     };
//     let bc: LineSegment3<f32> = LineSegment3 {
//         start: Vec3::new(x + dist_x, y - dist_y, z - dist_z),
//         end: Vec3::new(x + dist_x, y + dist_y, z - dist_z),
//     };

//     let cd = LineSegment3 {
//         start: Vec3::new(x + dist_x, y + dist_y, z - dist_z),
//         end: Vec3::new(x - dist_x, y + dist_y, z - dist_z),
//     };

//     let da: LineSegment3<f32> = LineSegment3 {
//         start: Vec3::new(x - dist_x, y + dist_y, z - dist_z),
//         end: Vec3::new(x - dist_x, y - dist_y, z - dist_z),
//     };

//     let ef = LineSegment3 {
//         start: Vec3::new(x - dist_x, y - dist_y, z + dist_z),
//         end: Vec3::new(x + dist_x, y - dist_y, z + dist_z),
//     };

//     let fg: LineSegment3<f32> = LineSegment3 {
//         start: Vec3::new(x + dist_x, y - dist_y, z + dist_z),
//         end: Vec3::new(x + dist_x, y + dist_y, z + dist_z),
//     };

//     let gh = LineSegment3 {
//         start: Vec3::new(x + dist_x, y + dist_y, z + dist_z),
//         end: Vec3::new(x - dist_x, y + dist_y, z + dist_z),
//     };

//     let he: LineSegment3<f32> = LineSegment3 {
//         start: Vec3::new(x - dist_x, y + dist_y, z + dist_z),
//         end: Vec3::new(x - dist_x, y - dist_y, z + dist_z),
//     };

//     let dh = LineSegment3 {
//         start: da.start,
//         end: he.start,
//     };

//     let cg = LineSegment3 {
//         start: cd.start,
//         end: gh.start,
//     };

//     let bf = LineSegment3 {
//         start: bc.start,
//         end: fg.start,
//     };

//     let ae = LineSegment3 {
//         start: ab.start,
//         end: ef.start,
//     };

//     mesh.extend_meshes(&[
//         line_mesh(ab, color),
//         line_mesh(bc, color),
//         line_mesh(cd, color),
//         line_mesh(da, color),
//         line_mesh(ef, color),
//         line_mesh(fg, color),
//         line_mesh(gh, color),
//         line_mesh(he, color),
//         line_mesh(dh, color),
//         line_mesh(cg, color),
//         line_mesh(bf, color),
//         line_mesh(ae, color),
//     ]);

//     mesh
// }

// pub fn line_mesh(segment: LineSegment3<f32>, color: [f32; 4]) -> Mesh<DebugVertex> {
//     box_along_line(segment, color, 0.1, 0.1)
// }

// /// ```text  
// ///       h-----g
// ///      /|    /|
// ///     / |  r/ |
// ///    /  |  /  |
// ///   /   e-/-- f
// ///  /   / /   /
// /// d-----c   /
// /// |  /  |  /
// /// | /q  | /
// /// |/    |/
// /// a-----b
// /// ```
// pub fn box_along_line(
//     segment: LineSegment3<f32>,
//     color: [f32; 4],
//     width: f32,
//     height: f32,
// ) -> Vec<DebugVertex>{
//     let mut mesh = Mesh::new();
//     let quad = |v1: Vec3<f32>, v2: Vec3<f32>, v3: Vec3<f32>, v4: Vec3<f32>| {
//         Quad::<DebugVertex>::new(
//             (v1, color).into(),
//             (v2, color).into(),
//             (v3, color).into(),
//             (v4, color).into(),
//         )
//     };

//     // Distance from the center of the segment to the edge of the box.
//     let dist_x = width / 2.0;
//     let dist_y = height / 2.0;

//     let (start_x, start_y, start_z) = segment.start.into_tuple();
//     let (end_x, end_y, end_z) = segment.end.into_tuple();

//     let a = Vec3::new(start_x - dist_x, start_y - dist_y, start_z);
//     let b = Vec3::new(start_x + dist_x, start_y - dist_y, start_z);
//     let c = Vec3::new(start_x + dist_x, start_y + dist_y, start_z);
//     let d = Vec3::new(start_x - dist_x, start_y + dist_y, start_z);

//     let e = Vec3::new(end_x + dist_x, end_y - dist_y, end_z);
//     let f = Vec3::new(end_x - dist_x, end_y - dist_y, end_z);
//     let g = Vec3::new(end_x - dist_x, end_y + dist_y, end_z);
//     let h = Vec3::new(end_x + dist_x, end_y + dist_y, end_z);

//     // -z
//     mesh.push_quad(quad(a, b, c, d));
//     // +z
//     mesh.push_quad(quad(f, g, h, e));
//     // -y
//     mesh.push_quad(quad(b, a, f, e));
//     // +y
//     mesh.push_quad(quad(h, g, d, c));
//     // +x
//     mesh.push_quad(quad(c, b, e, h));
//     // -x
//     mesh.push_quad(quad(a, d, g, f));
//     mesh
// }
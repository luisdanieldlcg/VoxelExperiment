use super::Vertex;

pub mod chunk;
pub mod debug;

/// Represents a quad.
///
/// A quad is a mesh that is composed of four vertices.
///
///```text
///    v4----------------v3
///     |                |
///     |                |
///     |     Quad       |
///     |                |
///     |                |
///    v1----------------v2
///```
///
/// ### Type parameters
///
/// * `V`: The vertex type.
pub struct Quad<V: Vertex> {
    v1: V,
    v2: V,
    v3: V,
    v4: V,
}

impl<V: Vertex> Quad<V> {
    /// Initializes a new `Quad` structure.
    pub fn new(v1: V, v2: V, v3: V, v4: V) -> Self {
        Self { v1, v2, v3, v4 }
    }
}

/// Represents a mesh that is stored on the CPU.
#[derive(Default)]
pub struct Mesh<V: Vertex> {
    /// `Vec` structure that stores the vertices.
    vertices: Vec<V>,
}

impl<V: Vertex> Mesh<V> {
    /// Pushes a quad to the mesh.
    ///
    /// The the winding order for assembling the quad is counter-clockwise.
    ///
    /// If you are not sure check out your pipeline configuration for [wgpu::FrontFace].
    ///
    ///```text
    ///    v4----------------v3
    ///     |                |
    ///     |                |
    ///     |     Quad       |
    ///     |                |
    ///     |                |
    ///    v1----------------v2
    ///```
    ///
    /// When using an index buffer, the vertices are pushed in the following order:
    /// - `[0, 1, 2, 2, 3, 0]`.
    ///
    /// Make sure to have set correctly the [wgpu::IndexFormat] in your [Vertex] implementation.
    pub fn push_quad(&mut self, quad: Quad<V>) {
        match V::INDEX_BUFFER {
            // The pipeline uses an index buffer.
            Some(_) => {
                // 0, 1, 2, 2, 3, 0 (counter-clockwise)
                self.vertices.push(quad.v1);
                self.vertices.push(quad.v2);
                self.vertices.push(quad.v3);
                self.vertices.push(quad.v4);
            },
            // The pipeline does not use an index buffer.
            None => {
                self.vertices.push(quad.v1);
                self.vertices.push(quad.v2);
                self.vertices.push(quad.v3);
                self.vertices.push(quad.v3);
                self.vertices.push(quad.v4);
                self.vertices.push(quad.v1);
            },
        };
    }

    pub fn extend_meshes(&mut self, meshes: &[Mesh<V>]) {
        for mesh in meshes {
            self.vertices.extend_from_slice(mesh.vertices());
        }
    }

    pub fn append(&mut self, vertices: &mut Vec<V>) {
        self.vertices.append(vertices);
    }

    /// Gives you a slice of the vertices.
    pub fn vertices(&self) -> &[V] {
        &self.vertices
    }

    pub fn vertices_mut(&mut self) -> &mut Vec<V> {
        &mut self.vertices
    }

    pub fn iter(&self) -> std::slice::Iter<V> {
        self.vertices.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<V> {
        self.vertices.iter_mut()
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }
}

// Allows to iterate over the vertices of the mesh.
impl<V: Vertex> IntoIterator for Mesh<V> {
    type Item = V;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vertices.into_iter()
    }
}

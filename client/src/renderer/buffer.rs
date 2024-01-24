use wgpu::util::DeviceExt;

/// Represents a GPU buffer.
///
/// It is a wrapper around [wgpu::Buffer].
/// This specific implementation allows to create and initialize a buffer in one step.
pub struct Buffer<T: Copy + bytemuck::Pod> {
    /// The underlying buffer handle.
    buf: wgpu::Buffer,
    /// The length of the buffer.
    len: u32,
    /// A phantom data field to make the compiler happy.
    ///
    /// It is needed because the generic type `T` is not used in the struct.
    /// However, it is very helpful to have a typed buffer.
    phantom: std::marker::PhantomData<T>,
}

impl<T: Copy + bytemuck::Pod> Buffer<T> {
    /// Creates a new [Buffer].
    ///
    /// The buffer is initialized with the given data.
    pub fn new(device: &wgpu::Device, usage: wgpu::BufferUsages, data: &[T]) -> Self {
        let descriptor = wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage,
        };
        Self {
            buf: device.create_buffer_init(&descriptor),
            phantom: std::marker::PhantomData,
            len: data.len() as u32,
        }
    }

    /// Write data into the buffer.
    ///
    /// If the data is empty it will do nothing to avoid
    /// unnecessary write calls.
    pub fn write(&self, queue: &wgpu::Queue, data: &[T]) {
        if data.is_empty() {
            return;
        }
        queue.write_buffer(&self.buf, 0, bytemuck::cast_slice(data))
    }

    /// Gives you the length of the buffer.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        self.len
    }
}

impl<T: Copy + bytemuck::Pod> std::ops::Deref for Buffer<T> {
    type Target = wgpu::Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

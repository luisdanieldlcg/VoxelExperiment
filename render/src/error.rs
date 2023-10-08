pub enum RenderError {
    DeviceRequest(wgpu::RequestDeviceError),
    AdapterNotFound,
    SurfaceError(wgpu::CreateSurfaceError),
}

impl From<wgpu::RequestDeviceError> for RenderError {
    fn from(value: wgpu::RequestDeviceError) -> Self {
        Self::DeviceRequest(value)
    }
}

impl From<wgpu::CreateSurfaceError> for RenderError {
    fn from(value: wgpu::CreateSurfaceError) -> Self {
        Self::SurfaceError(value)
    }
}

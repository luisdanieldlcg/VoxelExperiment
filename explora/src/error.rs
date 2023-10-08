#[derive(Debug)]
pub enum Error {
    Window(winit::error::OsError),
}

impl From<winit::error::OsError> for Error {
    fn from(value: winit::error::OsError) -> Self {
        Self::Window(value)
    }
}

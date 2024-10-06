use crate::{prelude::*, renderer::Renderer};

pub type CameraViewMut<'a> = UniqueViewMut<'a, Camera>;
pub type CameraView<'a> = UniqueView<'a, Camera>;

#[derive(Component, Unique)]
pub struct Camera {
    pub zoom: f64,
    pub x: f64,
    pub y: f64,
    pub buffer_id: Id,
    pub(super) _view_matrix: Mat4,
    pub(super) _proj_matrix: Mat4,
    pub(super) _buffer_data: [f32;32],
}

impl Camera {
    pub const Z_DEPTH:f32 = 100.0;

    pub fn new(renderer: &mut Renderer) -> Result<Self> {
        let buffer_id = renderer.create_buffer()?;
        Ok(Self { 
            zoom: 1.0,
            x: 0.0,
            y: 0.0,
            buffer_id,
            _view_matrix: Mat4::identity(),
            _proj_matrix: Mat4::identity(),
            _buffer_data: [0.0;32],
        })
    }

    pub fn buffer_data(&self) -> &[f32] {
        &self._buffer_data
    }
    pub fn view_matrix(&self) -> &Mat4 {
        &self._view_matrix
    }
    pub fn proj_matrix(&self) -> &Mat4 {
        &self._proj_matrix
    }

    pub fn view_proj_matrix(&self) -> Mat4 {
        self._proj_matrix * self._view_matrix
    }
}

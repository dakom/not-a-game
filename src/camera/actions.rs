use super::data::Camera;
use crate::prelude::*;

impl Camera {
    pub fn resize(&mut self, viewport_width: f64, viewport_height: f64) {
        let left = ((-viewport_width / (2.0 * self.zoom)) + self.x) as f32;
        let right = ((viewport_width / (2.0 * self.zoom)) + self.x) as f32;
        let bottom = ((-viewport_height / (2.0 * self.zoom)) + self.y) as f32;
        let top = ((viewport_height / (2.0 * self.zoom)) + self.y) as f32;

        self._proj_matrix =
            Mat4::new_orthographic(left, right, bottom, top, -Self::Z_DEPTH, Self::Z_DEPTH);

        self._view_matrix
            .write_to_vf32(&mut self._buffer_data[0..16]);
        self._proj_matrix
            .write_to_vf32(&mut self._buffer_data[16..32]);
    }
}

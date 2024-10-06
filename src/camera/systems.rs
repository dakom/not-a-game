use awsm_web::webgl::BufferUsage;

use crate::{prelude::*, renderer::{RendererViewMut, Renderer}};
use super::data::CameraView;

pub fn camera_update_ubo_sys(
    camera: CameraView, 
    mut renderer: RendererViewMut, 
) {
    renderer.upload_uniform_buffer_f32(
        camera.buffer_id,
        &camera.buffer_data(),
        BufferUsage::DynamicDraw,
    ).unwrap_ext();

    renderer.activate_uniform_buffer_loc(camera.buffer_id, Renderer::UBO_CAMERA);
}
use awsm_web::webgl::{BeginMode, BlendFactor, BufferData, BufferTarget, BufferUsage, GlToggle};

use super::data::Background;
use crate::{
    prelude::*,
    renderer::{buffers::Buffers, uvs::Uvs, Renderer},
};

impl Background {
    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width as f32;
        self.height = height as f32;
    }
}

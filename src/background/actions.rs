
use awsm_web::webgl::{BeginMode, GlToggle, BlendFactor, BufferData, BufferTarget, BufferUsage};

use crate::{prelude::*, renderer::{Renderer, uvs::Uvs, buffers::Buffers}};
use super::data::Background;

impl Background {
    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width as f32;
        self.height = height as f32;
    }
}
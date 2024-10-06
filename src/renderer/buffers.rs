use awsm_web::webgl::{WebGl2Renderer, BufferData, BufferTarget, BufferUsage};

use crate::prelude::*;
use super::Renderer;

pub struct Buffers {
    pub quad_geom: Id,
    pub collision_geom: Id,
    pub quad_uvs: Id,
    pub collision_uvs: Id,
}

impl Buffers {
    pub const QUAD_GEOM_UNIT: [f32; 8] = [
        0.0, 1.0, // top-left
        0.0, 0.0, //bottom-left
        1.0, 1.0, // top-right
        1.0, 0.0, // bottom-right
    ];
    pub fn new(gl: &mut WebGl2Renderer) -> Result<Self> {

        let quad_geom = gl.create_buffer()?;
        gl.upload_buffer(
            quad_geom,
            BufferData::new(
                &Buffers::QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            )
        )?;

        let quad_uvs = gl.create_buffer()?;
        let collision_geom = gl.create_buffer()?;
        let collision_uvs = gl.create_buffer()?;

        Ok(Self {
            quad_geom,
            quad_uvs,
            collision_geom,
            collision_uvs
        })
    }
}
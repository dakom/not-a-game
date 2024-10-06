use awsm_web::webgl::{AttributeOptions, DataType, NameOrLoc, VertexArray, WebGl2Renderer};

use super::{buffers::Buffers, Renderer};
use crate::prelude::*;

const QUAD_UNIT_VERTEX: &'static str = include_str!("./shaders/vertex/quad-unit.vert");
const QUAD_TEXTURE_FRAGMENT: &'static str = include_str!("./shaders/fragment/quad-texture.frag");
const COLLISION_VERTEX: &'static str = include_str!("./shaders/vertex/collision.vert");
const COLLISION_FRAGMENT: &'static str = include_str!("./shaders/fragment/collision.frag");

#[derive(Clone, Debug)]
pub struct Shaders {
    pub vertex: VertexShaders,
    pub fragment: FragmentShaders,
    pub programs: ShaderPrograms,
}

#[derive(Clone, Debug)]
pub struct VertexShaders {
    pub quad_unit: Id,
    pub collision: Id,
}

#[derive(Clone, Debug)]
pub struct FragmentShaders {
    pub quad_texture: Id,
    pub collision: Id,
}

#[derive(Clone, Debug)]
pub struct ShaderPrograms {
    pub sprite: ShaderProgram,
    pub collision: ShaderProgram,
}

#[derive(Clone, Debug)]
pub struct ShaderProgram {
    pub program_id: Id,
    pub vao_id: Id,
}

impl ShaderProgram {
    pub fn activate(&self, renderer: &mut Renderer) -> Result<()> {
        renderer.activate_program(self.program_id)?;
        renderer.activate_vertex_array(self.vao_id)?;
        Ok(())
    }
}

impl Shaders {
    pub fn compile(gl: &mut WebGl2Renderer, buffers: &Buffers) -> Result<Self> {
        let vertex = VertexShaders {
            quad_unit: gl.compile_shader(QUAD_UNIT_VERTEX, awsm_web::webgl::ShaderType::Vertex)?,
            collision: gl.compile_shader(COLLISION_VERTEX, awsm_web::webgl::ShaderType::Vertex)?,
        };

        let fragment = FragmentShaders {
            quad_texture: gl
                .compile_shader(QUAD_TEXTURE_FRAGMENT, awsm_web::webgl::ShaderType::Fragment)?,
            collision: gl
                .compile_shader(COLLISION_FRAGMENT, awsm_web::webgl::ShaderType::Fragment)?,
        };

        let sprite_shader = {
            let program_id = gl.compile_program(&[vertex.quad_unit, fragment.quad_texture])?;
            gl.init_uniform_buffer_name(program_id, "ubo_camera")?;

            let vao_id = gl.create_vertex_array()?;

            gl.assign_vertex_array(
                vao_id,
                None,
                &[
                    VertexArray {
                        attribute: NameOrLoc::Name("a_geom_vertex"),
                        buffer_id: buffers.quad_geom,
                        opts: AttributeOptions::new(2, DataType::Float),
                    },
                    VertexArray {
                        attribute: NameOrLoc::Name("a_uv_vertex"),
                        buffer_id: buffers.quad_uvs, // this will be overwritten
                        opts: AttributeOptions::new(2, DataType::Float),
                    },
                ],
            )?;

            ShaderProgram { program_id, vao_id }
        };

        let collision_shader = {
            let program_id = gl.compile_program(&[vertex.collision, fragment.collision])?;
            gl.init_uniform_buffer_name(program_id, "ubo_camera")?;

            let vao_id = gl.create_vertex_array()?;

            gl.assign_vertex_array(
                vao_id,
                None,
                &[
                    VertexArray {
                        attribute: NameOrLoc::Name("a_geom_vertex"),
                        buffer_id: buffers.collision_geom, // this will be overwritten
                        opts: AttributeOptions::new(2, DataType::Float),
                    },
                    VertexArray {
                        attribute: NameOrLoc::Name("a_uv_vertex"),
                        buffer_id: buffers.collision_uvs, // this will also be overwritten
                        opts: AttributeOptions::new(2, DataType::Float),
                    },
                ],
            )?;

            ShaderProgram { program_id, vao_id }
        };

        let programs = ShaderPrograms {
            sprite: sprite_shader,
            collision: collision_shader,
        };

        Ok(Self {
            vertex,
            fragment,
            programs,
        })
    }
}

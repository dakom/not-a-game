// note - the context must be created with antialias: false
// no idea why
use crate::prelude::*;
use awsm_web::webgl::{
    AttributeOptions, BeginMode, BlitFilter, Buffer, BufferData, BufferMask, BufferTarget,
    BufferUsage, DataType, DrawBuffer, FrameBufferAttachment, FrameBufferTarget,
    FrameBufferTextureTarget, GlToggle, Id, NameOrLoc, PartialWebGlFrameBuffer, PixelFormat,
    RenderBufferFormat, SimpleTextureOptions, TextureMagFilter, TextureMinFilter, TextureTarget,
    VertexArray, WebGl2Renderer, WebGlTextureSource,
};
use shipyard::*;

use super::Renderer;

//pub type DrawBuffersView<'a> = UniqueView<'a, Option<DrawBuffers>>;
//pub type DrawBuffersViewMut<'a> = UniqueViewMut<'a, Option<DrawBuffers>>;

impl Renderer {
    pub fn resize_framebuffers(&mut self) -> Result<()> {
        if let Some(framebuffers) = self.framebuffers.as_mut() {
            framebuffers.destroy(&mut self.gl)?;
        }
        self.framebuffers = None;

        let (_, _, width, height) = self.get_viewport();
        if width < 1 || height < 1 {
            return Ok(());
        }

        self.framebuffers = Some(FrameBuffers::new(
            &mut self.gl,
            Some(MultisampleMode::Msaa),
        )?);
        Ok(())
    }

    pub fn pre_draw(&mut self) -> Result<bool> {
        if let Some(framebuffers) = &self.framebuffers {
            self.bind_framebuffer(framebuffers.fbo_draw.id, FrameBufferTarget::DrawFrameBuffer)?;
            self.clear_draw_buffer_fi(Buffer::DepthStencil, 0, 1.0, 0);
            self.clear_draw_buffer_vf32_values(Buffer::Color, 0, &[0.0, 0.0, 0.0, 0.0]);

            self.gl.gl.color_mask(true, true, true, true);
            self.gl.gl.depth_mask(true);
            self.gl.toggle(GlToggle::DepthTest, true);
            self.gl.toggle(GlToggle::StencilTest, false);
            self.clear(&[
                BufferMask::ColorBufferBit,
                BufferMask::DepthBufferBit,
                BufferMask::StencilBufferBit,
            ]);

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn post_draw(&mut self) -> Result<bool> {
        let (_, _, width, height) = self.get_viewport();
        if let Some(framebuffers) = &self.framebuffers {
            self.bind_framebuffer(framebuffers.fbo_draw.id, FrameBufferTarget::ReadFrameBuffer)?;

            // multisampling
            // i.e. to downsample from the msaa into single-sample fbo
            // and that can't be done directly into the front buffer
            if let Some(fbo_multisample) = &framebuffers.fbo_multisample {
                self.bind_framebuffer(fbo_multisample.id, FrameBufferTarget::DrawFrameBuffer)?;
                self.blit_framebuffer(
                    0,
                    0,
                    framebuffers.width,
                    framebuffers.height,
                    0,
                    0,
                    framebuffers.width,
                    framebuffers.height,
                    BufferMask::ColorBufferBit,
                    BlitFilter::Nearest,
                );

                self.bind_framebuffer(fbo_multisample.id, FrameBufferTarget::ReadFrameBuffer)?;
            }

            self.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
            self.blit_framebuffer(
                0,
                0,
                framebuffers.width,
                framebuffers.height,
                0,
                0,
                framebuffers.width,
                framebuffers.height,
                BufferMask::ColorBufferBit,
                BlitFilter::Nearest,
            );

            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Component, Unique)]
pub struct FrameBuffers {
    pub width: u32,
    pub height: u32,
    pub fbo_draw: FrameBuffer,
    pub fbo_multisample: Option<FrameBuffer>,
    pub fbo_collision: FrameBuffer,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MultisampleMode {
    Msaa,
}

//see: https://stackoverflow.com/questions/21841483/webgl-using-framebuffers-for-picking-multiple-objects
//https://stackoverflow.com/questions/51101023/render-to-16bits-unsigned-integer-2d-texture-in-webgl2
//
impl FrameBuffers {
    pub fn new(gl: &mut WebGl2Renderer, multisample_mode: Option<MultisampleMode>) -> Result<Self> {
        let (_, _, width, height) = gl.get_viewport();
        let msaa = multisample_mode == Some(MultisampleMode::Msaa);

        let fbo_draw = FrameBuffer::new(gl)?
            .build_depth(gl, width, height, FrameBufferIdKind::Render, msaa)?
            .build_color(gl, width, height, FrameBufferIdKind::Render, msaa)?
            .validate(gl)?;

        gl.draw_buffers(&vec![DrawBuffer::Color0])?;
        fbo_draw.release(gl);

        let fbo_multisample = match multisample_mode {
            None => None,
            Some(mode) => {
                match mode {
                    MultisampleMode::Msaa => {
                        // multisample blit target is just color for downsampling, no need for depth and this is not multisampled
                        let fbo_main_multisample = FrameBuffer::new(gl)?
                            .build_color(gl, width, height, FrameBufferIdKind::Render, false)?
                            .validate(gl)?;

                        fbo_main_multisample.release(gl);
                        Some(fbo_main_multisample)
                    }
                }
            }
        };

        let fbo_collision = FrameBuffer::new(gl)?
            .build_stencil(gl, width, height, FrameBufferIdKind::Render)?
            .validate(gl)?;

        fbo_collision.release(gl);

        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        Ok(Self {
            width,
            height,
            fbo_draw,
            fbo_multisample,
            fbo_collision,
        })
    }

    pub fn destroy(&mut self, mut gl: &mut WebGl2Renderer) -> Result<()> {
        self.fbo_draw.destroy(&mut gl)?;
        if let Some(mut fbo_multisample) = self.fbo_multisample.take() {
            fbo_multisample.destroy(&mut gl)?;
        }
        Ok(())
    }
}

pub struct FrameBuffer {
    pub id: Id,
    pub depth: Option<FrameBufferId>,
    pub color: Option<FrameBufferId>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameBufferId {
    pub kind: FrameBufferIdKind,
    pub id: Id,
}

impl FrameBufferId {
    pub fn destroy(&mut self, mut gl: &mut WebGl2Renderer) -> Result<()> {
        match self.kind {
            FrameBufferIdKind::Render => gl.delete_renderbuffer(self.id).map_err(|err| err.into()),
            FrameBufferIdKind::Texture => gl.delete_texture(self.id).map_err(|err| err.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameBufferIdKind {
    Render,
    Texture,
}

impl FrameBuffer {
    pub fn new(gl: &mut WebGl2Renderer) -> Result<Self> {
        let id = gl.create_framebuffer()?;

        Ok(Self {
            id,
            depth: None,
            color: None,
        })
    }

    pub fn build_stencil(
        mut self,
        gl: &mut WebGl2Renderer,
        width: u32,
        height: u32,
        kind: FrameBufferIdKind,
    ) -> Result<Self> {
        let depth_id = match kind {
            FrameBufferIdKind::Render => {
                let depth_id = gl.create_renderbuffer()?;

                gl.assign_renderbuffer_storage(
                    depth_id,
                    RenderBufferFormat::Depth24Stencil8,
                    width,
                    height,
                )?;
                gl.assign_framebuffer_renderbuffer(
                    self.id,
                    depth_id,
                    FrameBufferTarget::DrawFrameBuffer,
                    FrameBufferAttachment::DepthStencil,
                )?;

                depth_id
            }
            // untested...
            FrameBufferIdKind::Texture => {
                let depth_id = make_texture(gl, width, height)?;
                gl.assign_framebuffer_texture_2d(
                    self.id,
                    depth_id,
                    FrameBufferTarget::DrawFrameBuffer,
                    FrameBufferAttachment::DepthStencil,
                    FrameBufferTextureTarget::Texture2d,
                )?;

                depth_id
            }
        };

        self.depth = Some(FrameBufferId { kind, id: depth_id });

        Ok(self)
    }

    pub fn build_depth(
        mut self,
        gl: &mut WebGl2Renderer,
        width: u32,
        height: u32,
        kind: FrameBufferIdKind,
        multisample: bool,
    ) -> Result<Self> {
        let depth_id = match kind {
            FrameBufferIdKind::Render => {
                let depth_id = gl.create_renderbuffer()?;

                if multisample {
                    gl.assign_renderbuffer_storage_multisample_max(
                        depth_id,
                        RenderBufferFormat::DepthComponent32f,
                        width,
                        height,
                    )?;
                } else {
                    gl.assign_renderbuffer_storage(
                        depth_id,
                        RenderBufferFormat::DepthComponent32f,
                        width,
                        height,
                    )?;
                }
                gl.assign_framebuffer_renderbuffer(
                    self.id,
                    depth_id,
                    FrameBufferTarget::DrawFrameBuffer,
                    FrameBufferAttachment::Depth,
                )?;

                depth_id
            }
            // untested...
            FrameBufferIdKind::Texture => {
                if multisample {
                    return Err(anyhow!("todo: multisample texture not support"));
                }
                let depth_id = make_texture(gl, width, height)?;
                gl.assign_framebuffer_texture_2d(
                    self.id,
                    depth_id,
                    FrameBufferTarget::DrawFrameBuffer,
                    FrameBufferAttachment::Depth,
                    FrameBufferTextureTarget::Texture2d,
                )?;

                depth_id
            }
        };

        self.depth = Some(FrameBufferId { kind, id: depth_id });

        Ok(self)
    }

    pub fn build_color(
        mut self,
        gl: &mut WebGl2Renderer,
        width: u32,
        height: u32,
        kind: FrameBufferIdKind,
        multisample: bool,
    ) -> Result<Self> {
        let color_id = match kind {
            FrameBufferIdKind::Render => {
                let color_id = gl.create_renderbuffer()?;
                // Rgb8 because canvas is alpha: false.
                // According to the spec:
                // "When blitting to the color attachment of the WebGL context's default back buffer, a context created with alpha:false is considered to have internal format RGB8, while a context created with alpha:true is considered to have internal format RGBA8."
                if multisample {
                    gl.assign_renderbuffer_storage_multisample_max(
                        color_id,
                        RenderBufferFormat::Rgb8,
                        width,
                        height,
                    )?;
                } else {
                    gl.assign_renderbuffer_storage(
                        color_id,
                        RenderBufferFormat::Rgb8,
                        width,
                        height,
                    )?;
                }
                gl.assign_framebuffer_renderbuffer(
                    self.id,
                    color_id,
                    FrameBufferTarget::DrawFrameBuffer,
                    FrameBufferAttachment::Color0,
                )?;

                color_id
            }
            FrameBufferIdKind::Texture => {
                if multisample {
                    return Err(anyhow!("todo: multisample texture not support"));
                }
                let color_id = make_texture(gl, width, height)?;
                gl.assign_framebuffer_texture_2d(
                    self.id,
                    color_id,
                    FrameBufferTarget::DrawFrameBuffer,
                    FrameBufferAttachment::Color0,
                    FrameBufferTextureTarget::Texture2d,
                )?;

                color_id
            }
        };

        self.color = Some(FrameBufferId { kind, id: color_id });

        Ok(self)
    }

    pub fn release(&self, gl: &mut WebGl2Renderer) {
        gl.release_texture_target(TextureTarget::Texture2d);
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::ReadFrameBuffer);
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
    }

    pub fn validate(mut self, gl: &mut WebGl2Renderer) -> Result<Self> {
        //make sure we're all good
        gl.check_framebuffer_status(FrameBufferTarget::DrawFrameBuffer)?;

        Ok(self)
    }

    pub fn destroy(&mut self, mut gl: &mut WebGl2Renderer) -> Result<()> {
        gl.delete_framebuffer(self.id)?;

        if let Some(mut depth) = self.depth {
            depth.destroy(gl)?;
        }
        if let Some(mut color) = self.color {
            color.destroy(gl)?;
        }

        Ok(())
    }
}

fn make_texture(gl: &mut WebGl2Renderer, width: u32, height: u32) -> Result<Id> {
    let id = gl.create_texture()?;

    gl.assign_simple_texture(
        id,
        TextureTarget::Texture2d,
        &SimpleTextureOptions {
            flip_y: Some(false),
            filter_min: Some(TextureMinFilter::Nearest),
            filter_mag: Some(TextureMagFilter::Nearest),
            pixel_format: PixelFormat::Rgba,
            ..SimpleTextureOptions::default()
        },
        &WebGlTextureSource::EmptyBufferView(width, height, 0),
    )?;

    Ok(id)
}

use std::{ffi::CString, str::Utf8Error, string::FromUtf8Error};

use raylib::color::Color;
use raylib::math::Vector2;
use raylib::prelude::RaylibTexture2D;
use raylib::{
    math::Rectangle,
    prelude::{RaylibDraw, RaylibShaderModeExt},
    shaders::Shader,
    texture::RenderTexture2D,
    RaylibHandle, RaylibThread,
};
use rust_embed::EmbeddedFile;
use tracing::info;

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error(transparent)]
    UtfConversionError(#[from] FromUtf8Error),
}

pub struct ShaderWrapper {
    shader: Shader,
}

impl ShaderWrapper {
    /// Construct a new shader wrapper.
    pub fn new(
        vertex_shader: Option<EmbeddedFile>,
        fragment_shader: Option<EmbeddedFile>,
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<Self, ShaderError> {
        let vertex_shader_code = vertex_shader.map(|file| String::from_utf8(file.data.to_vec()));
        let fragment_shader_code =
            fragment_shader.map(|file| String::from_utf8(file.data.to_vec()));

        Ok(Self {
            shader: load_shader_from_heap(
                raylib,
                &thread,
                match vertex_shader_code {
                    Some(result) => match result {
                        Ok(code) => Some(code),
                        Err(err) => return Err(ShaderError::UtfConversionError(err)),
                    },
                    None => None,
                },
                match fragment_shader_code {
                    Some(result) => match result {
                        Ok(code) => Some(code),
                        Err(err) => return Err(ShaderError::UtfConversionError(err)),
                    },
                    None => None,
                },
            ),
        })
    }

    pub fn process_texture_and_render<H>(
        &self,
        raylib: &mut H,
        thread: &RaylibThread,
        texture: &RenderTexture2D,
    ) where
        H: RaylibShaderModeExt + RaylibDraw,
    {
        puffin::profile_function!();
        // Create a shader context to work under
        let mut shader_context = raylib.begin_shader_mode(&self.shader);

        // Blit the texture
        shader_context.draw_texture_pro(
            &texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: texture.width() as f32,
                height: (texture.height() as f32) * -1.0,
            },
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: texture.width() as f32,
                height: texture.height() as f32,
            },
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}

/// Too lazy to write this upstream
fn load_shader_from_heap(
    handle: &mut RaylibHandle,
    thread: &RaylibThread,
    vs: Option<String>,
    fs: Option<String>,
) -> Shader {
    let vs_code = vs.unwrap_or(String::new());
    let vs_code_str = vs_code.as_str();
    let fs_code = fs.unwrap_or(String::new());
    let fs_code_str = fs_code.as_str();
    handle.load_shader_code(
        thread,
        match vs_code.len() {
            0 => None,
            _ => Some(vs_code_str),
        },
        match fs_code.len() {
            0 => None,
            _ => Some(fs_code_str),
        },
    )
}

use std::collections::HashMap;
use std::{ffi::CString, string::FromUtf8Error};

use raylib::color::Color;
use raylib::math::Vector2;
use raylib::prelude::RaylibTexture2D;
use raylib::shaders::ShaderV;
use raylib::{
    math::Rectangle,
    prelude::{RaylibDraw, RaylibShaderModeExt},
    shaders::Shader,
    texture::RenderTexture2D,
    RaylibHandle, RaylibThread,
};
use rust_embed::EmbeddedFile;

use crate::utilities::non_ref_raylib::HackedRaylibHandle;

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error(transparent)]
    UtfConversionError(#[from] FromUtf8Error),
    #[error(transparent)]
    ShaderVarNameNulError(#[from] std::ffi::NulError),
    #[error("Shader variable name not valid: {0}")]
    ShaderVarNameError(String),
}

/// Utility wrapper around shader FFI
pub struct ShaderWrapper {
    shader: Shader,
    variables: HashMap<String, i32>,
}

impl ShaderWrapper {
    /// Construct a new shader wrapper.
    pub fn new(
        vertex_shader: Option<EmbeddedFile>,
        fragment_shader: Option<EmbeddedFile>,
        variable_names: Vec<&str>,
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<Self, ShaderError> {
        // Load shader code from files
        let vertex_shader_code = vertex_shader.map(|file| String::from_utf8(file.data.to_vec()));
        let fragment_shader_code =
            fragment_shader.map(|file| String::from_utf8(file.data.to_vec()));

        // Create shader
        let shader = load_shader_from_heap(
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
        );

        // Create connections between CPU and GPU
        let mut variables = HashMap::new();
        for variable_name in variable_names {

            // I know what I'm doing here. We can skip this error
            #[allow(unsafe_code)]
            variables.insert(variable_name.to_string(), unsafe {
                raylib::ffi::GetShaderLocation(*shader, CString::new(variable_name)?.as_ptr())
            });
        }

        Ok(Self { shader, variables })
    }

    /// Handles rendering a texture to the screen via the shader. If run inside another shader context, this *should* chain with it.
    pub fn process_texture_and_render(
        &self,
        raylib: &mut HackedRaylibHandle,
        _thread: &RaylibThread,
        texture: &RenderTexture2D,
    )
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

    /// Set a variable in the shader.
    pub fn set_variable<S>(&mut self, name: &str, value: S) -> Result<(), ShaderError>
    where
        S: ShaderV,
    {
        puffin::profile_function!();
        if let Some(ptr) = self.variables.get(name) {
            self.shader.set_shader_value(*ptr, value);
            Ok(())
        } else {
            Err(ShaderError::ShaderVarNameError(name.to_string()))
        }
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

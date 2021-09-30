use raylib::ffi::RenderTexture;

/// Renders everything in the draw function to a texture
#[allow(unsafe_code)]
pub fn render_to_texture<Func>(texture: &mut RenderTexture, draw_fn: Func) where Func: FnOnce() {
    puffin::profile_function!();
    unsafe {
        raylib::ffi::BeginTextureMode(*texture);
    }
    draw_fn();
    unsafe {
        raylib::ffi::EndTextureMode();
    }
}

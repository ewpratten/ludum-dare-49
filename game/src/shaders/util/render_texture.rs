use raylib::ffi::RenderTexture;

/// Renders everything in the draw function to a texture
pub fn render_to_texture<Func>(texture: &mut RenderTexture, draw_fn: Func) where Func: FnOnce() {
    unsafe {
        raylib::ffi::BeginTextureMode(*texture);
    }
    draw_fn();
    unsafe {
        raylib::ffi::EndTextureMode();
    }
}

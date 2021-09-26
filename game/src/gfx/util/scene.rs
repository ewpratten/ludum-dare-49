use raylib::prelude::RaylibDrawHandle;

/// Defines any renderable scene
pub trait Scene<Context, Error> {
    /// Render the hud layer (screen-space rendering)
    fn render_hud(
        &mut self,
        gfx: &mut RaylibDrawHandle,
        delta_seconds: f64,
        ctx: &Context,
    ) -> Result<(), Error>;

    /// Render the world layer (world-space rendering)
    fn render_world(
        &mut self,
        gfx: &mut RaylibDrawHandle,
        delta_seconds: f64,
        ctx: &Context,
    ) -> Result<(), Error>;
}

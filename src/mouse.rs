//! Mouse utility functions.

use sdl2::mouse;
use context::Context;
use error::GameResult;
use graphics;
use graphics::Point2;

/// Get whether or not the mouse is "grabbed", ie, confined to the window.
pub fn get_grabbed(ctx: &Context) -> bool {
    graphics::get_window(ctx).grab()
}

/// Set whether or not the mouse is "grabbed", ie, confined to the window.
pub fn set_grabbed(ctx: &mut Context, grabbed: bool) {
    graphics::get_window_mut(ctx).set_grab(grabbed)
}

/// Get whether or not the mouse is in relative mode.
///
/// In relative mode, the cursor is hidden and doesn't move when the mouse
/// does, but relative motion events are still generated.  This is useful
/// for things such as implementing mouselook in an FPS.
pub fn get_relative_mode(ctx: &Context) -> bool {
    ctx.sdl_context.mouse().relative_mouse_mode()
}

/// Set whether or not the mouse is in relative mode.
pub fn set_relative_mode(ctx: &Context, mode: bool) {
    ctx.sdl_context.mouse().set_relative_mouse_mode(mode)
}

/// Get the current position of the mouse cursor, in pixels.
/// Uses strictly window-only coordinates.
pub fn get_position(ctx: &Context) -> GameResult<Point2> {
    let event_pump = &ctx.sdl_context.event_pump()?;
    let mouse = mouse::MouseState::new(event_pump);
    let x = mouse.x() as f32;
    let y = mouse.y() as f32;
    Ok(Point2::new(x, y))
}

/// Set the current position of the mouse cursor, in pixels.
/// Uses strictly window-only coordinates.
pub fn set_position(ctx: &Context, point: Point2) {
    let window = graphics::get_window(ctx);
    ctx.sdl_context
        .mouse()
        .warp_mouse_in_window(window, point.x as i32, point.y as i32)
}

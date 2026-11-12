use macroquad::input::is_quit_requested;
use macroquad::time::{draw_fps, get_fps, get_frame_time, get_time};
use macroquad::window::{
    request_new_screen_size, screen_dpi_scale, screen_height, screen_width, set_fullscreen,
};
use steel::rvals::{Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;

use crate::types::{steel_to_bool, steel_to_f32};

pub fn register(module: &mut BuiltInModule) {
    module
        .register_fn("screen-width", || -> f64 { screen_width() as f64 })
        .register_fn("screen-height", || -> f64 { screen_height() as f64 })
        .register_fn("screen-dpi-scale", || -> f64 { screen_dpi_scale() as f64 })
        .register_fn("get-frame-time", || -> f64 { get_frame_time() as f64 })
        .register_fn("get-fps", || -> i32 { get_fps() })
        .register_fn("get-time", || -> f64 { get_time() })
        .register_fn(
            "request-new-screen-size",
            |w: SteelVal, h: SteelVal| -> Result<()> {
                request_new_screen_size(steel_to_f32(&w)?, steel_to_f32(&h)?);
                Ok(())
            },
        )
        .register_fn("set-fullscreen", |fullscreen: SteelVal| -> Result<()> {
            set_fullscreen(steel_to_bool(&fullscreen)?);
            Ok(())
        })
        .register_fn("is-quit-requested?", || -> bool { is_quit_requested() })
        .register_fn("draw-fps", || {
            draw_fps();
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_registration() {
        let mut module = BuiltInModule::new("macroquad");
        register(&mut module);
        let names = module.names();
        for expected in [
            "screen-width",
            "screen-height",
            "screen-dpi-scale",
            "get-frame-time",
            "get-fps",
            "get-time",
            "request-new-screen-size",
            "set-fullscreen",
            "is-quit-requested?",
            "draw-fps",
        ] {
            assert!(
                names.contains(&expected.to_string()),
                "missing window fn: {expected}"
            );
        }
    }
}

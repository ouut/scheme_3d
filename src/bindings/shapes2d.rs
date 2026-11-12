use macroquad::shapes::{
    draw_arc, draw_circle, draw_circle_lines, draw_ellipse, draw_ellipse_lines, draw_line,
    draw_poly, draw_poly_lines, draw_rectangle, draw_rectangle_lines, draw_triangle,
    draw_triangle_lines,
};
use macroquad::window::clear_background;
use steel::rvals::{Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;

use crate::types::{steel_to_color, steel_to_f32, steel_to_u8, steel_to_vec2};

pub fn register(module: &mut BuiltInModule) {
    module
        .register_fn("clear-background", |color: SteelVal| -> Result<()> {
            clear_background(steel_to_color(&color)?);
            Ok(())
        })
        .register_fn(
            "draw-line",
            |x1: SteelVal,
             y1: SteelVal,
             x2: SteelVal,
             y2: SteelVal,
             thickness: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_line(
                    steel_to_f32(&x1)?,
                    steel_to_f32(&y1)?,
                    steel_to_f32(&x2)?,
                    steel_to_f32(&y2)?,
                    steel_to_f32(&thickness)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-rectangle",
            |x: SteelVal, y: SteelVal, w: SteelVal, h: SteelVal, color: SteelVal| -> Result<()> {
                draw_rectangle(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&w)?,
                    steel_to_f32(&h)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-rectangle-lines",
            |x: SteelVal,
             y: SteelVal,
             w: SteelVal,
             h: SteelVal,
             thickness: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_rectangle_lines(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&w)?,
                    steel_to_f32(&h)?,
                    steel_to_f32(&thickness)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-circle",
            |x: SteelVal, y: SteelVal, r: SteelVal, color: SteelVal| -> Result<()> {
                draw_circle(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&r)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-circle-lines",
            |x: SteelVal,
             y: SteelVal,
             r: SteelVal,
             thickness: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_circle_lines(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&r)?,
                    steel_to_f32(&thickness)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-triangle",
            |v1: SteelVal, v2: SteelVal, v3: SteelVal, color: SteelVal| -> Result<()> {
                draw_triangle(
                    steel_to_vec2(&v1)?,
                    steel_to_vec2(&v2)?,
                    steel_to_vec2(&v3)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-triangle-lines",
            |v1: SteelVal,
             v2: SteelVal,
             v3: SteelVal,
             thickness: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_triangle_lines(
                    steel_to_vec2(&v1)?,
                    steel_to_vec2(&v2)?,
                    steel_to_vec2(&v3)?,
                    steel_to_f32(&thickness)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-poly",
            |x: SteelVal,
             y: SteelVal,
             sides: SteelVal,
             radius: SteelVal,
             rotation: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_poly(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_u8(&sides)?,
                    steel_to_f32(&radius)?,
                    steel_to_f32(&rotation)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-poly-lines",
            |x: SteelVal,
             y: SteelVal,
             sides: SteelVal,
             radius: SteelVal,
             rotation: SteelVal,
             thickness: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_poly_lines(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_u8(&sides)?,
                    steel_to_f32(&radius)?,
                    steel_to_f32(&rotation)?,
                    steel_to_f32(&thickness)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-ellipse",
            |x: SteelVal,
             y: SteelVal,
             w: SteelVal,
             h: SteelVal,
             rotation: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_ellipse(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&w)?,
                    steel_to_f32(&h)?,
                    steel_to_f32(&rotation)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-ellipse-lines",
            |x: SteelVal,
             y: SteelVal,
             w: SteelVal,
             h: SteelVal,
             rotation: SteelVal,
             thickness: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_ellipse_lines(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&w)?,
                    steel_to_f32(&h)?,
                    steel_to_f32(&rotation)?,
                    steel_to_f32(&thickness)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-arc",
            |x: SteelVal,
             y: SteelVal,
             sides: SteelVal,
             radius: SteelVal,
             rotation: SteelVal,
             thickness: SteelVal,
             arc: SteelVal,
             color: SteelVal|
             -> Result<()> {
                draw_arc(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_u8(&sides)?,
                    steel_to_f32(&radius)?,
                    steel_to_f32(&rotation)?,
                    steel_to_f32(&thickness)?,
                    steel_to_f32(&arc)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shapes2d_registration() {
        let mut module = BuiltInModule::new("macroquad");
        register(&mut module);
        let names = module.names();
        for expected in [
            "clear-background",
            "draw-line",
            "draw-rectangle",
            "draw-rectangle-lines",
            "draw-circle",
            "draw-circle-lines",
            "draw-triangle",
            "draw-triangle-lines",
            "draw-poly",
            "draw-poly-lines",
            "draw-ellipse",
            "draw-ellipse-lines",
            "draw-arc",
        ] {
            assert!(
                names.contains(&expected.to_string()),
                "missing 2D shape fn: {expected}"
            );
        }
    }
}

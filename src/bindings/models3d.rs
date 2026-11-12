use macroquad::models::{
    draw_cube, draw_cube_wires, draw_cylinder, draw_cylinder_ex, draw_cylinder_wires, draw_grid,
    draw_line_3d, draw_plane, draw_sphere, draw_sphere_ex, draw_sphere_wires, DrawCylinderParams,
    DrawSphereParams,
};
use macroquad::prelude::DrawMode;
use steel::rvals::{Result, SteelVal};
use steel::steel_vm::builtin::{Arity, BuiltInModule};
use steel::steel_vm::register_fn::RegisterFn;
use steel::stop;

use crate::types::{
    steel_to_bool, steel_to_color, steel_to_f32, steel_to_u32, steel_to_usize, steel_to_vec2,
    steel_to_vec3,
};

fn draw_cylinder_native(args: &[SteelVal]) -> Result<SteelVal> {
    match args.len() {
        5 => {
            draw_cylinder(
                steel_to_vec3(&args[0])?,
                steel_to_f32(&args[1])?,
                steel_to_f32(&args[2])?,
                steel_to_f32(&args[3])?,
                None,
                steel_to_color(&args[4])?,
            );
            Ok(SteelVal::Void)
        }
        6 => {
            draw_cylinder_ex(
                steel_to_vec3(&args[0])?,
                steel_to_f32(&args[1])?,
                steel_to_f32(&args[2])?,
                steel_to_f32(&args[3])?,
                None,
                steel_to_color(&args[5])?,
                DrawCylinderParams {
                    sides: steel_to_usize(&args[4])?,
                    draw_mode: DrawMode::Triangles,
                },
            );
            Ok(SteelVal::Void)
        }
        n => stop!(ArityMismatch => "draw-cylinder expected 5 or 6 arguments, got {}", n),
    }
}

fn draw_cylinder_wires_native(args: &[SteelVal]) -> Result<SteelVal> {
    match args.len() {
        5 => {
            draw_cylinder_wires(
                steel_to_vec3(&args[0])?,
                steel_to_f32(&args[1])?,
                steel_to_f32(&args[2])?,
                steel_to_f32(&args[3])?,
                None,
                steel_to_color(&args[4])?,
            );
            Ok(SteelVal::Void)
        }
        6 => {
            draw_cylinder_ex(
                steel_to_vec3(&args[0])?,
                steel_to_f32(&args[1])?,
                steel_to_f32(&args[2])?,
                steel_to_f32(&args[3])?,
                None,
                steel_to_color(&args[5])?,
                DrawCylinderParams {
                    sides: steel_to_usize(&args[4])?,
                    draw_mode: DrawMode::Lines,
                },
            );
            Ok(SteelVal::Void)
        }
        n => stop!(ArityMismatch => "draw-cylinder-wires expected 5 or 6 arguments, got {}", n),
    }
}

pub fn register(module: &mut BuiltInModule) {
    module
        .register_fn(
            "draw-cube",
            |pos: SteelVal, size: SteelVal, color: SteelVal| -> Result<()> {
                draw_cube(
                    steel_to_vec3(&pos)?,
                    steel_to_vec3(&size)?,
                    None,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-cube-wires",
            |pos: SteelVal, size: SteelVal, color: SteelVal| -> Result<()> {
                draw_cube_wires(
                    steel_to_vec3(&pos)?,
                    steel_to_vec3(&size)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-sphere",
            |center: SteelVal, radius: SteelVal, color: SteelVal| -> Result<()> {
                draw_sphere(
                    steel_to_vec3(&center)?,
                    steel_to_f32(&radius)?,
                    None,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-sphere-wires",
            |center: SteelVal, radius: SteelVal, color: SteelVal| -> Result<()> {
                draw_sphere_wires(
                    steel_to_vec3(&center)?,
                    steel_to_f32(&radius)?,
                    None,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-sphere-ex",
            |center: SteelVal,
             radius: SteelVal,
             rings: SteelVal,
             slices: SteelVal,
             wires: SteelVal,
             color: SteelVal|
             -> Result<()> {
                let draw_mode = if steel_to_bool(&wires)? {
                    DrawMode::Lines
                } else {
                    DrawMode::Triangles
                };
                draw_sphere_ex(
                    steel_to_vec3(&center)?,
                    steel_to_f32(&radius)?,
                    None,
                    steel_to_color(&color)?,
                    DrawSphereParams {
                        rings: steel_to_usize(&rings)?,
                        slices: steel_to_usize(&slices)?,
                        draw_mode,
                    },
                );
                Ok(())
            },
        )
        .register_native_fn("draw-cylinder", draw_cylinder_native, Arity::Range(5, 6))
        .register_native_fn(
            "draw-cylinder-wires",
            draw_cylinder_wires_native,
            Arity::Range(5, 6),
        )
        .register_fn(
            "draw-cylinder-ex",
            |pos: SteelVal,
             radius_top: SteelVal,
             radius_bottom: SteelVal,
             height: SteelVal,
             sides: SteelVal,
             wires: SteelVal,
             color: SteelVal|
             -> Result<()> {
                let draw_mode = if steel_to_bool(&wires)? {
                    DrawMode::Lines
                } else {
                    DrawMode::Triangles
                };
                draw_cylinder_ex(
                    steel_to_vec3(&pos)?,
                    steel_to_f32(&radius_top)?,
                    steel_to_f32(&radius_bottom)?,
                    steel_to_f32(&height)?,
                    None,
                    steel_to_color(&color)?,
                    DrawCylinderParams {
                        sides: steel_to_usize(&sides)?,
                        draw_mode,
                    },
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-plane",
            |center: SteelVal, size: SteelVal, color: SteelVal| -> Result<()> {
                draw_plane(
                    steel_to_vec3(&center)?,
                    steel_to_vec2(&size)?,
                    None,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-grid",
            |slices: SteelVal,
             spacing: SteelVal,
             axes_color: SteelVal,
             other_color: SteelVal|
             -> Result<()> {
                draw_grid(
                    steel_to_u32(&slices)?,
                    steel_to_f32(&spacing)?,
                    steel_to_color(&axes_color)?,
                    steel_to_color(&other_color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-line-3d",
            |start: SteelVal, end: SteelVal, color: SteelVal| -> Result<()> {
                draw_line_3d(
                    steel_to_vec3(&start)?,
                    steel_to_vec3(&end)?,
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
    fn test_models3d_registration() {
        let mut module = BuiltInModule::new("macroquad");
        register(&mut module);
        let names = module.names();
        for expected in [
            "draw-cube",
            "draw-cube-wires",
            "draw-sphere",
            "draw-sphere-wires",
            "draw-sphere-ex",
            "draw-cylinder",
            "draw-cylinder-wires",
            "draw-cylinder-ex",
            "draw-plane",
            "draw-grid",
            "draw-line-3d",
        ] {
            assert!(
                names.contains(&expected.to_string()),
                "missing 3D primitive fn: {expected}"
            );
        }
    }
}

use macroquad::color::{colors, Color};
use steel::rvals::{Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;

use crate::types::{
    color_into_steelval, steel_to_color, steel_to_f32, steel_to_u32, steel_to_u8,
};

pub fn register(module: &mut BuiltInModule) {
    let color_constants = [
        ("LIGHTGRAY", colors::LIGHTGRAY),
        ("GRAY", colors::GRAY),
        ("DARKGRAY", colors::DARKGRAY),
        ("YELLOW", colors::YELLOW),
        ("GOLD", colors::GOLD),
        ("ORANGE", colors::ORANGE),
        ("PINK", colors::PINK),
        ("RED", colors::RED),
        ("MAROON", colors::MAROON),
        ("GREEN", colors::GREEN),
        ("LIME", colors::LIME),
        ("DARKGREEN", colors::DARKGREEN),
        ("SKYBLUE", colors::SKYBLUE),
        ("BLUE", colors::BLUE),
        ("DARKBLUE", colors::DARKBLUE),
        ("PURPLE", colors::PURPLE),
        ("VIOLET", colors::VIOLET),
        ("DARKPURPLE", colors::DARKPURPLE),
        ("BEIGE", colors::BEIGE),
        ("BROWN", colors::BROWN),
        ("DARKBROWN", colors::DARKBROWN),
        ("WHITE", colors::WHITE),
        ("BLACK", colors::BLACK),
        ("BLANK", colors::BLANK),
        ("MAGENTA", colors::MAGENTA),
    ];

    for (name, color) in color_constants {
        if let Ok(val) = color_into_steelval(color) {
            module.register_value(name, val);
        }
    }

    module
        .register_fn(
            "rgba",
            |r: SteelVal, g: SteelVal, b: SteelVal, a: SteelVal| -> Result<SteelVal> {
                color_into_steelval(Color::new(
                    steel_to_f32(&r)?,
                    steel_to_f32(&g)?,
                    steel_to_f32(&b)?,
                    steel_to_f32(&a)?,
                ))
            },
        )
        .register_fn(
            "rgb",
            |r: SteelVal, g: SteelVal, b: SteelVal| -> Result<SteelVal> {
                color_into_steelval(Color::new(
                    steel_to_f32(&r)?,
                    steel_to_f32(&g)?,
                    steel_to_f32(&b)?,
                    1.0,
                ))
            },
        )
        .register_fn(
            "rgba8",
            |r: SteelVal, g: SteelVal, b: SteelVal, a: SteelVal| -> Result<SteelVal> {
                color_into_steelval(Color::from_rgba(
                    steel_to_u8(&r)?,
                    steel_to_u8(&g)?,
                    steel_to_u8(&b)?,
                    steel_to_u8(&a)?,
                ))
            },
        )
        .register_fn(
            "rgb8",
            |r: SteelVal, g: SteelVal, b: SteelVal| -> Result<SteelVal> {
                color_into_steelval(Color::from_rgba(
                    steel_to_u8(&r)?,
                    steel_to_u8(&g)?,
                    steel_to_u8(&b)?,
                    255,
                ))
            },
        )
        .register_fn("color-from-hex", |hex: SteelVal| -> Result<SteelVal> {
            color_into_steelval(Color::from_hex(steel_to_u32(&hex)?))
        })
        .register_fn(
            "color-with-alpha",
            |c: SteelVal, alpha: SteelVal| -> Result<SteelVal> {
                color_into_steelval(steel_to_color(&c)?.with_alpha(steel_to_f32(&alpha)?))
            },
        )
        .register_fn("color-r", |c: SteelVal| -> Result<f64> {
            Ok(steel_to_color(&c)?.r as f64)
        })
        .register_fn("color-g", |c: SteelVal| -> Result<f64> {
            Ok(steel_to_color(&c)?.g as f64)
        })
        .register_fn("color-b", |c: SteelVal| -> Result<f64> {
            Ok(steel_to_color(&c)?.b as f64)
        })
        .register_fn("color-a", |c: SteelVal| -> Result<f64> {
            Ok(steel_to_color(&c)?.a as f64)
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{register_types, MqColor};
    use steel::steel_vm::engine::Engine;

    #[test]
    fn test_color_constructors_and_constants() {
        let mut vm = Engine::new();
        let mut module = BuiltInModule::new("macroquad");
        register_types(&mut module);
        register(&mut module);
        vm.register_module(module);
        vm.run("(require-builtin macroquad)").unwrap();

        let res = vm
            .run(
                r#"
                (define c1 (rgba 1 0.5 0 1))
                (define c2 (rgba8 255 0 0 255))
                (define c3 (color-from-hex #xFF0000))
                (define c4 (color-with-alpha '(1 0 0) 0.25))
                (list
                    (color? c1)
                    (color? WHITE)
                    (equal? c2 c3)
                    (color-r c1)
                    (color-g c1)
                    (color-a c4))
                "#,
            )
            .unwrap();
        let SteelVal::ListV(items) = res.last().unwrap() else {
            panic!("expected list");
        };
        let v: Vec<SteelVal> = items.iter().cloned().collect();
        assert_eq!(v[0], SteelVal::BoolV(true));
        assert_eq!(v[1], SteelVal::BoolV(true));
        assert_eq!(v[2], SteelVal::BoolV(true));
        assert_eq!(v[3], SteelVal::NumV(1.0));
        assert_eq!(v[4], SteelVal::NumV(0.5));
        assert_eq!(v[5], SteelVal::NumV(0.25));

        assert!(vm.run("(rgba8 300 0 0 255)").is_err());
        let _ = MqColor(colors::WHITE);
    }
}

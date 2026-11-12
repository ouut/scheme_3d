use macroquad::text::{draw_multiline_text, draw_text, measure_text};
use steel::rvals::{Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel::stop;

use crate::types::{steel_to_color, steel_to_f32, steel_to_optional_f32};

fn steel_to_text(val: &SteelVal) -> Result<String> {
    match val {
        SteelVal::StringV(s) => Ok(s.as_str().to_owned()),
        SteelVal::SymbolV(s) => Ok(s.as_str().to_owned()),
        _ => stop!(TypeMismatch => "expected string for text argument, found: {}", val),
    }
}

fn validate_font_size_u16(val: &SteelVal) -> Result<u16> {
    let size = steel_to_f32(val)?;
    if !size.is_finite() || size < 0.0 || size > u16::MAX as f32 {
        stop!(
            ConversionError => "font-size out of valid range (0..=65535), found: {}",
            size
        );
    }
    Ok(size.round() as u16)
}

pub fn register(module: &mut BuiltInModule) {
    module
        .register_fn(
            "draw-text",
            |text: SteelVal,
             x: SteelVal,
             y: SteelVal,
             font_size: SteelVal,
             color: SteelVal|
             -> Result<()> {
                let s = steel_to_text(&text)?;
                draw_text(
                    &s,
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&font_size)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "draw-multiline-text",
            |text: SteelVal,
             x: SteelVal,
             y: SteelVal,
             font_size: SteelVal,
             line_factor: SteelVal,
             color: SteelVal|
             -> Result<()> {
                let s = steel_to_text(&text)?;
                draw_multiline_text(
                    &s,
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&font_size)?,
                    steel_to_optional_f32(&line_factor)?,
                    steel_to_color(&color)?,
                );
                Ok(())
            },
        )
        .register_fn(
            "measure-text",
            |text: SteelVal, font_size: SteelVal| -> Result<SteelVal> {
                let s = steel_to_text(&text)?;
                let sz = validate_font_size_u16(&font_size)?;
                let dims = measure_text(&s, None, sz, 1.0);
                Ok(SteelVal::ListV(
                    vec![
                        SteelVal::NumV(dims.width as f64),
                        SteelVal::NumV(dims.height as f64),
                        SteelVal::NumV(dims.offset_y as f64),
                    ]
                    .into(),
                ))
            },
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_helpers_and_registration() {
        assert_eq!(validate_font_size_u16(&SteelVal::IntV(24)).unwrap(), 24);
        assert_eq!(validate_font_size_u16(&SteelVal::NumV(18.4)).unwrap(), 18);
        assert!(validate_font_size_u16(&SteelVal::IntV(-1)).is_err());

        let mut module = BuiltInModule::new("macroquad");
        register(&mut module);
        let names = module.names();
        for expected in ["draw-text", "draw-multiline-text", "measure-text"] {
            assert!(names.contains(&expected.to_string()));
        }
    }
}

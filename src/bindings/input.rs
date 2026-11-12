use macroquad::input::{
    get_char_pressed, get_last_key_pressed, is_any_key_down, is_key_down, is_key_pressed,
    is_key_released, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
    mouse_delta_position, mouse_position, mouse_position_local, mouse_wheel, set_cursor_grab,
    show_mouse, KeyCode, MouseButton,
};
use macroquad::math::Vec2;
use steel::rvals::{Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel::stop;

use crate::types::{steel_to_bool, steel_to_symbol_or_str, vec2_into_steelval};

pub fn symbol_to_key_code(name: &str) -> std::result::Result<KeyCode, String> {
    let normalized = name.trim().to_ascii_lowercase();
    let code = match normalized.as_str() {
        "space" => KeyCode::Space,
        "apostrophe" | "quote" => KeyCode::Apostrophe,
        "comma" => KeyCode::Comma,
        "minus" => KeyCode::Minus,
        "period" => KeyCode::Period,
        "slash" => KeyCode::Slash,
        "key0" | "0" => KeyCode::Key0,
        "key1" | "1" => KeyCode::Key1,
        "key2" | "2" => KeyCode::Key2,
        "key3" | "3" => KeyCode::Key3,
        "key4" | "4" => KeyCode::Key4,
        "key5" | "5" => KeyCode::Key5,
        "key6" | "6" => KeyCode::Key6,
        "key7" | "7" => KeyCode::Key7,
        "key8" | "8" => KeyCode::Key8,
        "key9" | "9" => KeyCode::Key9,
        "semicolon" => KeyCode::Semicolon,
        "equal" => KeyCode::Equal,
        "a" => KeyCode::A,
        "b" => KeyCode::B,
        "c" => KeyCode::C,
        "d" => KeyCode::D,
        "e" => KeyCode::E,
        "f" => KeyCode::F,
        "g" => KeyCode::G,
        "h" => KeyCode::H,
        "i" => KeyCode::I,
        "j" => KeyCode::J,
        "k" => KeyCode::K,
        "l" => KeyCode::L,
        "m" => KeyCode::M,
        "n" => KeyCode::N,
        "o" => KeyCode::O,
        "p" => KeyCode::P,
        "q" => KeyCode::Q,
        "r" => KeyCode::R,
        "s" => KeyCode::S,
        "t" => KeyCode::T,
        "u" => KeyCode::U,
        "v" => KeyCode::V,
        "w" => KeyCode::W,
        "x" => KeyCode::X,
        "y" => KeyCode::Y,
        "z" => KeyCode::Z,
        "left-bracket" => KeyCode::LeftBracket,
        "backslash" => KeyCode::Backslash,
        "right-bracket" => KeyCode::RightBracket,
        "grave-accent" | "backtick" => KeyCode::GraveAccent,
        "world1" => KeyCode::World1,
        "world2" => KeyCode::World2,
        "escape" | "esc" => KeyCode::Escape,
        "enter" | "return" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "backspace" => KeyCode::Backspace,
        "insert" => KeyCode::Insert,
        "delete" => KeyCode::Delete,
        "right" => KeyCode::Right,
        "left" => KeyCode::Left,
        "down" => KeyCode::Down,
        "up" => KeyCode::Up,
        "page-up" => KeyCode::PageUp,
        "page-down" => KeyCode::PageDown,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "caps-lock" => KeyCode::CapsLock,
        "scroll-lock" => KeyCode::ScrollLock,
        "num-lock" => KeyCode::NumLock,
        "print-screen" => KeyCode::PrintScreen,
        "pause" => KeyCode::Pause,
        "f1" => KeyCode::F1,
        "f2" => KeyCode::F2,
        "f3" => KeyCode::F3,
        "f4" => KeyCode::F4,
        "f5" => KeyCode::F5,
        "f6" => KeyCode::F6,
        "f7" => KeyCode::F7,
        "f8" => KeyCode::F8,
        "f9" => KeyCode::F9,
        "f10" => KeyCode::F10,
        "f11" => KeyCode::F11,
        "f12" => KeyCode::F12,
        "f13" => KeyCode::F13,
        "f14" => KeyCode::F14,
        "f15" => KeyCode::F15,
        "f16" => KeyCode::F16,
        "f17" => KeyCode::F17,
        "f18" => KeyCode::F18,
        "f19" => KeyCode::F19,
        "f20" => KeyCode::F20,
        "f21" => KeyCode::F21,
        "f22" => KeyCode::F22,
        "f23" => KeyCode::F23,
        "f24" => KeyCode::F24,
        "f25" => KeyCode::F25,
        "kp0" => KeyCode::Kp0,
        "kp1" => KeyCode::Kp1,
        "kp2" => KeyCode::Kp2,
        "kp3" => KeyCode::Kp3,
        "kp4" => KeyCode::Kp4,
        "kp5" => KeyCode::Kp5,
        "kp6" => KeyCode::Kp6,
        "kp7" => KeyCode::Kp7,
        "kp8" => KeyCode::Kp8,
        "kp9" => KeyCode::Kp9,
        "kp-decimal" => KeyCode::KpDecimal,
        "kp-divide" => KeyCode::KpDivide,
        "kp-multiply" => KeyCode::KpMultiply,
        "kp-subtract" => KeyCode::KpSubtract,
        "kp-add" => KeyCode::KpAdd,
        "kp-enter" => KeyCode::KpEnter,
        "kp-equal" => KeyCode::KpEqual,
        "left-shift" | "lshift" | "shift" => KeyCode::LeftShift,
        "left-control" | "lctrl" | "ctrl" => KeyCode::LeftControl,
        "left-alt" | "lalt" | "alt" => KeyCode::LeftAlt,
        "left-super" | "lsuper" => KeyCode::LeftSuper,
        "right-shift" | "rshift" => KeyCode::RightShift,
        "right-control" | "rctrl" => KeyCode::RightControl,
        "right-alt" | "ralt" => KeyCode::RightAlt,
        "right-super" | "rsuper" => KeyCode::RightSuper,
        "menu" => KeyCode::Menu,
        "back" => KeyCode::Back,
        "unknown" => KeyCode::Unknown,
        other => return Err(format!("unknown key symbol: '{other}'")),
    };
    Ok(code)
}

pub fn key_code_to_symbol(key: KeyCode) -> &'static str {
    match key {
        KeyCode::Space => "space",
        KeyCode::Apostrophe => "apostrophe",
        KeyCode::Comma => "comma",
        KeyCode::Minus => "minus",
        KeyCode::Period => "period",
        KeyCode::Slash => "slash",
        KeyCode::Key0 => "key0",
        KeyCode::Key1 => "key1",
        KeyCode::Key2 => "key2",
        KeyCode::Key3 => "key3",
        KeyCode::Key4 => "key4",
        KeyCode::Key5 => "key5",
        KeyCode::Key6 => "key6",
        KeyCode::Key7 => "key7",
        KeyCode::Key8 => "key8",
        KeyCode::Key9 => "key9",
        KeyCode::Semicolon => "semicolon",
        KeyCode::Equal => "equal",
        KeyCode::A => "a",
        KeyCode::B => "b",
        KeyCode::C => "c",
        KeyCode::D => "d",
        KeyCode::E => "e",
        KeyCode::F => "f",
        KeyCode::G => "g",
        KeyCode::H => "h",
        KeyCode::I => "i",
        KeyCode::J => "j",
        KeyCode::K => "k",
        KeyCode::L => "l",
        KeyCode::M => "m",
        KeyCode::N => "n",
        KeyCode::O => "o",
        KeyCode::P => "p",
        KeyCode::Q => "q",
        KeyCode::R => "r",
        KeyCode::S => "s",
        KeyCode::T => "t",
        KeyCode::U => "u",
        KeyCode::V => "v",
        KeyCode::W => "w",
        KeyCode::X => "x",
        KeyCode::Y => "y",
        KeyCode::Z => "z",
        KeyCode::LeftBracket => "left-bracket",
        KeyCode::Backslash => "backslash",
        KeyCode::RightBracket => "right-bracket",
        KeyCode::GraveAccent => "grave-accent",
        KeyCode::World1 => "world1",
        KeyCode::World2 => "world2",
        KeyCode::Escape => "escape",
        KeyCode::Enter => "enter",
        KeyCode::Tab => "tab",
        KeyCode::Backspace => "backspace",
        KeyCode::Insert => "insert",
        KeyCode::Delete => "delete",
        KeyCode::Right => "right",
        KeyCode::Left => "left",
        KeyCode::Down => "down",
        KeyCode::Up => "up",
        KeyCode::PageUp => "page-up",
        KeyCode::PageDown => "page-down",
        KeyCode::Home => "home",
        KeyCode::End => "end",
        KeyCode::CapsLock => "caps-lock",
        KeyCode::ScrollLock => "scroll-lock",
        KeyCode::NumLock => "num-lock",
        KeyCode::PrintScreen => "print-screen",
        KeyCode::Pause => "pause",
        KeyCode::F1 => "f1",
        KeyCode::F2 => "f2",
        KeyCode::F3 => "f3",
        KeyCode::F4 => "f4",
        KeyCode::F5 => "f5",
        KeyCode::F6 => "f6",
        KeyCode::F7 => "f7",
        KeyCode::F8 => "f8",
        KeyCode::F9 => "f9",
        KeyCode::F10 => "f10",
        KeyCode::F11 => "f11",
        KeyCode::F12 => "f12",
        KeyCode::F13 => "f13",
        KeyCode::F14 => "f14",
        KeyCode::F15 => "f15",
        KeyCode::F16 => "f16",
        KeyCode::F17 => "f17",
        KeyCode::F18 => "f18",
        KeyCode::F19 => "f19",
        KeyCode::F20 => "f20",
        KeyCode::F21 => "f21",
        KeyCode::F22 => "f22",
        KeyCode::F23 => "f23",
        KeyCode::F24 => "f24",
        KeyCode::F25 => "f25",
        KeyCode::Kp0 => "kp0",
        KeyCode::Kp1 => "kp1",
        KeyCode::Kp2 => "kp2",
        KeyCode::Kp3 => "kp3",
        KeyCode::Kp4 => "kp4",
        KeyCode::Kp5 => "kp5",
        KeyCode::Kp6 => "kp6",
        KeyCode::Kp7 => "kp7",
        KeyCode::Kp8 => "kp8",
        KeyCode::Kp9 => "kp9",
        KeyCode::KpDecimal => "kp-decimal",
        KeyCode::KpDivide => "kp-divide",
        KeyCode::KpMultiply => "kp-multiply",
        KeyCode::KpSubtract => "kp-subtract",
        KeyCode::KpAdd => "kp-add",
        KeyCode::KpEnter => "kp-enter",
        KeyCode::KpEqual => "kp-equal",
        KeyCode::LeftShift => "left-shift",
        KeyCode::LeftControl => "left-control",
        KeyCode::LeftAlt => "left-alt",
        KeyCode::LeftSuper => "left-super",
        KeyCode::RightShift => "right-shift",
        KeyCode::RightControl => "right-control",
        KeyCode::RightAlt => "right-alt",
        KeyCode::RightSuper => "right-super",
        KeyCode::Menu => "menu",
        KeyCode::Back => "back",
        KeyCode::Unknown => "unknown",
    }
}

pub fn symbol_to_mouse_button(name: &str) -> std::result::Result<MouseButton, String> {
    match name.trim().to_ascii_lowercase().as_str() {
        "left" => Ok(MouseButton::Left),
        "middle" => Ok(MouseButton::Middle),
        "right" => Ok(MouseButton::Right),
        "unknown" => Ok(MouseButton::Unknown),
        other => Err(format!(
            "unknown mouse button symbol: '{other}' (expected 'left, 'middle, or 'right)"
        )),
    }
}

pub fn mouse_button_to_symbol(btn: MouseButton) -> &'static str {
    match btn {
        MouseButton::Left => "left",
        MouseButton::Middle => "middle",
        MouseButton::Right => "right",
        MouseButton::Unknown => "unknown",
    }
}

fn parse_key(val: &SteelVal) -> Result<KeyCode> {
    let s = steel_to_symbol_or_str(val)?;
    symbol_to_key_code(&s).map_err(|msg| steel::SteelErr::new(steel::rerrs::ErrorKind::Generic, msg))
}

fn parse_mouse_btn(val: &SteelVal) -> Result<MouseButton> {
    let s = steel_to_symbol_or_str(val)?;
    symbol_to_mouse_button(&s)
        .map_err(|msg| steel::SteelErr::new(steel::rerrs::ErrorKind::Generic, msg))
}

pub fn register(module: &mut BuiltInModule) {
    module
        .register_fn("is-key-down?", |key: SteelVal| -> Result<bool> {
            Ok(is_key_down(parse_key(&key)?))
        })
        .register_fn("is-key-pressed?", |key: SteelVal| -> Result<bool> {
            Ok(is_key_pressed(parse_key(&key)?))
        })
        .register_fn("is-key-released?", |key: SteelVal| -> Result<bool> {
            Ok(is_key_released(parse_key(&key)?))
        })
        .register_fn("is-any-key-down?", || -> bool { is_any_key_down() })
        .register_fn("get-last-key-pressed", || -> SteelVal {
            match get_last_key_pressed() {
                Some(k) => SteelVal::SymbolV(key_code_to_symbol(k).into()),
                None => SteelVal::BoolV(false),
            }
        })
        .register_fn("get-char-pressed", || -> SteelVal {
            match get_char_pressed() {
                Some(ch) => SteelVal::CharV(ch),
                None => SteelVal::BoolV(false),
            }
        })
        .register_fn("is-mouse-button-down?", |btn: SteelVal| -> Result<bool> {
            Ok(is_mouse_button_down(parse_mouse_btn(&btn)?))
        })
        .register_fn(
            "is-mouse-button-pressed?",
            |btn: SteelVal| -> Result<bool> { Ok(is_mouse_button_pressed(parse_mouse_btn(&btn)?)) },
        )
        .register_fn(
            "is-mouse-button-released?",
            |btn: SteelVal| -> Result<bool> {
                Ok(is_mouse_button_released(parse_mouse_btn(&btn)?))
            },
        )
        .register_fn("mouse-position", || -> Result<SteelVal> {
            let (x, y) = mouse_position();
            vec2_into_steelval(Vec2::new(x, y))
        })
        .register_fn("mouse-position-local", || -> Result<SteelVal> {
            vec2_into_steelval(mouse_position_local())
        })
        .register_fn("mouse-delta-position", || -> Result<SteelVal> {
            vec2_into_steelval(mouse_delta_position())
        })
        .register_fn("mouse-wheel", || -> Result<SteelVal> {
            let (x, y) = mouse_wheel();
            vec2_into_steelval(Vec2::new(x, y))
        })
        .register_fn("set-cursor-grab", |grab: SteelVal| -> Result<()> {
            set_cursor_grab(steel_to_bool(&grab)?);
            let _ = || -> Result<()> { stop!(Generic => "unreachable") };
            Ok(())
        })
        .register_fn("show-mouse", |show: SteelVal| -> Result<()> {
            show_mouse(steel_to_bool(&show)?);
            Ok(())
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_KEYS: &[KeyCode] = &[
        KeyCode::Space,
        KeyCode::Apostrophe,
        KeyCode::Comma,
        KeyCode::Minus,
        KeyCode::Period,
        KeyCode::Slash,
        KeyCode::Key0,
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
        KeyCode::Semicolon,
        KeyCode::Equal,
        KeyCode::A,
        KeyCode::B,
        KeyCode::C,
        KeyCode::D,
        KeyCode::E,
        KeyCode::F,
        KeyCode::G,
        KeyCode::H,
        KeyCode::I,
        KeyCode::J,
        KeyCode::K,
        KeyCode::L,
        KeyCode::M,
        KeyCode::N,
        KeyCode::O,
        KeyCode::P,
        KeyCode::Q,
        KeyCode::R,
        KeyCode::S,
        KeyCode::T,
        KeyCode::U,
        KeyCode::V,
        KeyCode::W,
        KeyCode::X,
        KeyCode::Y,
        KeyCode::Z,
        KeyCode::LeftBracket,
        KeyCode::Backslash,
        KeyCode::RightBracket,
        KeyCode::GraveAccent,
        KeyCode::World1,
        KeyCode::World2,
        KeyCode::Escape,
        KeyCode::Enter,
        KeyCode::Tab,
        KeyCode::Backspace,
        KeyCode::Insert,
        KeyCode::Delete,
        KeyCode::Right,
        KeyCode::Left,
        KeyCode::Down,
        KeyCode::Up,
        KeyCode::PageUp,
        KeyCode::PageDown,
        KeyCode::Home,
        KeyCode::End,
        KeyCode::CapsLock,
        KeyCode::ScrollLock,
        KeyCode::NumLock,
        KeyCode::PrintScreen,
        KeyCode::Pause,
        KeyCode::F1,
        KeyCode::F2,
        KeyCode::F3,
        KeyCode::F4,
        KeyCode::F5,
        KeyCode::F6,
        KeyCode::F7,
        KeyCode::F8,
        KeyCode::F9,
        KeyCode::F10,
        KeyCode::F11,
        KeyCode::F12,
        KeyCode::F13,
        KeyCode::F14,
        KeyCode::F15,
        KeyCode::F16,
        KeyCode::F17,
        KeyCode::F18,
        KeyCode::F19,
        KeyCode::F20,
        KeyCode::F21,
        KeyCode::F22,
        KeyCode::F23,
        KeyCode::F24,
        KeyCode::F25,
        KeyCode::Kp0,
        KeyCode::Kp1,
        KeyCode::Kp2,
        KeyCode::Kp3,
        KeyCode::Kp4,
        KeyCode::Kp5,
        KeyCode::Kp6,
        KeyCode::Kp7,
        KeyCode::Kp8,
        KeyCode::Kp9,
        KeyCode::KpDecimal,
        KeyCode::KpDivide,
        KeyCode::KpMultiply,
        KeyCode::KpSubtract,
        KeyCode::KpAdd,
        KeyCode::KpEnter,
        KeyCode::KpEqual,
        KeyCode::LeftShift,
        KeyCode::LeftControl,
        KeyCode::LeftAlt,
        KeyCode::LeftSuper,
        KeyCode::RightShift,
        KeyCode::RightControl,
        KeyCode::RightAlt,
        KeyCode::RightSuper,
        KeyCode::Menu,
        KeyCode::Back,
        KeyCode::Unknown,
    ];

    #[test]
    fn test_all_keycode_roundtrips_and_aliases() {
        assert_eq!(ALL_KEYS.len(), 122);
        for &key in ALL_KEYS {
            let sym = key_code_to_symbol(key);
            assert_eq!(symbol_to_key_code(sym).unwrap(), key);
            assert_eq!(
                symbol_to_key_code(&sym.to_ascii_uppercase()).unwrap(),
                key
            );
        }

        assert_eq!(symbol_to_key_code("0").unwrap(), KeyCode::Key0);
        assert_eq!(symbol_to_key_code("9").unwrap(), KeyCode::Key9);
        assert_eq!(symbol_to_key_code("esc").unwrap(), KeyCode::Escape);
        assert_eq!(symbol_to_key_code("return").unwrap(), KeyCode::Enter);
        assert_eq!(symbol_to_key_code("shift").unwrap(), KeyCode::LeftShift);
        assert_eq!(symbol_to_key_code("ctrl").unwrap(), KeyCode::LeftControl);
        assert!(symbol_to_key_code("not-a-key").is_err());
    }

    #[test]
    fn test_mouse_button_roundtrips() {
        for btn in [
            MouseButton::Left,
            MouseButton::Middle,
            MouseButton::Right,
            MouseButton::Unknown,
        ] {
            let sym = mouse_button_to_symbol(btn);
            assert_eq!(symbol_to_mouse_button(sym).unwrap(), btn);
        }
        assert!(symbol_to_mouse_button("thumb").is_err());
    }
}

use macroquad::camera::{Camera2D, Camera3D, Projection};
use macroquad::color::Color;
use macroquad::math::{Vec2, Vec3};
pub use steel::rvals::{FromSteelVal, IntoSteelVal};
use steel::rvals::{as_underlying_type, Custom, CustomType, Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::stop;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MqVec2(pub Vec2);

impl Custom for MqVec2 {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(format!("#<vec2 ({}, {})>", self.0.x, self.0.y)))
    }

    fn equality_hint(&self, other: &dyn CustomType) -> bool {
        as_underlying_type::<Self>(other)
            .map(|o| self == o)
            .unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MqVec3(pub Vec3);

impl Custom for MqVec3 {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(format!(
            "#<vec3 ({}, {}, {})>",
            self.0.x, self.0.y, self.0.z
        )))
    }

    fn equality_hint(&self, other: &dyn CustomType) -> bool {
        as_underlying_type::<Self>(other)
            .map(|o| self == o)
            .unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MqColor(pub Color);

impl Custom for MqColor {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(format!(
            "#<color ({:.3}, {:.3}, {:.3}, {:.3})>",
            self.0.r, self.0.g, self.0.b, self.0.a
        )))
    }

    fn equality_hint(&self, other: &dyn CustomType) -> bool {
        as_underlying_type::<Self>(other)
            .map(|o| self == o)
            .unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MqProjection {
    #[default]
    Perspective,
    Orthographic,
}

impl From<Projection> for MqProjection {
    fn from(value: Projection) -> Self {
        match value {
            Projection::Perspective => Self::Perspective,
            Projection::Orthographics => Self::Orthographic,
        }
    }
}

impl From<MqProjection> for Projection {
    fn from(value: MqProjection) -> Self {
        match value {
            MqProjection::Perspective => Projection::Perspective,
            MqProjection::Orthographic => Projection::Orthographics,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MqCamera2D {
    pub rotation: f32,
    pub zoom: Vec2,
    pub target: Vec2,
    pub offset: Vec2,
    pub viewport: Option<(i32, i32, i32, i32)>,
}

impl Default for MqCamera2D {
    fn default() -> Self {
        let def = Camera2D::default();
        Self {
            rotation: def.rotation,
            zoom: def.zoom,
            target: def.target,
            offset: def.offset,
            viewport: def.viewport,
        }
    }
}

impl MqCamera2D {
    pub fn to_macroquad(&self) -> Camera2D {
        Camera2D {
            rotation: self.rotation,
            zoom: self.zoom,
            target: self.target,
            offset: self.offset,
            render_target: None,
            viewport: self.viewport,
        }
    }
}

impl Custom for MqCamera2D {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(format!(
            "#<camera-2d target:({:.2}, {:.2}) zoom:({:.2}, {:.2}) rot:{:.2}>",
            self.target.x, self.target.y, self.zoom.x, self.zoom.y, self.rotation
        )))
    }

    fn equality_hint(&self, other: &dyn CustomType) -> bool {
        as_underlying_type::<Self>(other)
            .map(|o| self == o)
            .unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MqCamera3D {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
    pub aspect: Option<f32>,
    pub projection: MqProjection,
    pub viewport: Option<(i32, i32, i32, i32)>,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for MqCamera3D {
    fn default() -> Self {
        let def = Camera3D::default();
        Self {
            position: def.position,
            target: def.target,
            up: def.up,
            fovy: def.fovy,
            aspect: def.aspect,
            projection: def.projection.into(),
            viewport: def.viewport,
            z_near: def.z_near,
            z_far: def.z_far,
        }
    }
}

impl MqCamera3D {
    pub fn to_macroquad(&self) -> Camera3D {
        Camera3D {
            position: self.position,
            target: self.target,
            up: self.up,
            fovy: self.fovy,
            aspect: self.aspect,
            projection: self.projection.into(),
            render_target: None,
            viewport: self.viewport,
            z_near: self.z_near,
            z_far: self.z_far,
        }
    }
}

impl Custom for MqCamera3D {
    fn fmt(&self) -> Option<std::result::Result<String, std::fmt::Error>> {
        Some(Ok(format!(
            "#<camera-3d pos:({:.2}, {:.2}, {:.2}) target:({:.2}, {:.2}, {:.2}) fovy:{:.3}>",
            self.position.x,
            self.position.y,
            self.position.z,
            self.target.x,
            self.target.y,
            self.target.z,
            self.fovy
        )))
    }

    fn equality_hint(&self, other: &dyn CustomType) -> bool {
        as_underlying_type::<Self>(other)
            .map(|o| self == o)
            .unwrap_or(false)
    }
}

pub fn register_types(module: &mut BuiltInModule) {
    module
        .register_type::<MqVec2>("vec2?")
        .register_type::<MqVec3>("vec3?")
        .register_type::<MqColor>("color?")
        .register_type::<MqCamera2D>("camera-2d?")
        .register_type::<MqCamera3D>("camera-3d?");
}

pub fn steel_to_f64(val: &SteelVal) -> Result<f64> {
    match val {
        SteelVal::NumV(n) => Ok(*n),
        SteelVal::IntV(i) => Ok(*i as f64),
        SteelVal::Rational(r) => Ok(*r.numer() as f64 / *r.denom() as f64),
        SteelVal::BigNum(b) => b
            .to_string()
            .parse::<f64>()
            .map_err(|_| steel::SteelErr::new(
                steel::rerrs::ErrorKind::ConversionError,
                format!("Unable to convert bignum to f64: {val}"),
            )),
        SteelVal::BigRational(r) => {
            let num = r
                .numer()
                .to_string()
                .parse::<f64>()
                .map_err(|_| steel::SteelErr::new(
                    steel::rerrs::ErrorKind::ConversionError,
                    format!("Unable to convert bigrational numerator to f64: {val}"),
                ))?;
            let den = r
                .denom()
                .to_string()
                .parse::<f64>()
                .map_err(|_| steel::SteelErr::new(
                    steel::rerrs::ErrorKind::ConversionError,
                    format!("Unable to convert bigrational denominator to f64: {val}"),
                ))?;
            Ok(num / den)
        }
        _ => stop!(ConversionError => "expected real number, found: {}", val),
    }
}

pub fn steel_to_f32(val: &SteelVal) -> Result<f32> {
    Ok(steel_to_f64(val)? as f32)
}

pub fn steel_to_i64(val: &SteelVal) -> Result<i64> {
    match val {
        SteelVal::IntV(i) => Ok(*i as i64),
        SteelVal::NumV(n) if n.is_finite() && n.fract() == 0.0 => {
            if *n < i64::MIN as f64 || *n > i64::MAX as f64 {
                stop!(ConversionError => "number out of i64 range: {}", n);
            }
            Ok(*n as i64)
        }
        SteelVal::Rational(r) if *r.denom() == 1 => Ok(*r.numer() as i64),
        SteelVal::BigNum(b) => b.as_ref().try_into().map_err(|_| {
            steel::SteelErr::new(
                steel::rerrs::ErrorKind::ConversionError,
                format!("bignum out of i64 range: {val}"),
            )
        }),
        _ => stop!(ConversionError => "expected integer, found: {}", val),
    }
}

pub fn steel_to_i32(val: &SteelVal) -> Result<i32> {
    let n = steel_to_i64(val)?;
    i32::try_from(n).map_err(|_| {
        steel::SteelErr::new(
            steel::rerrs::ErrorKind::ConversionError,
            format!("integer out of i32 range: {n}"),
        )
    })
}

pub fn steel_to_usize(val: &SteelVal) -> Result<usize> {
    let n = steel_to_i64(val)?;
    usize::try_from(n).map_err(|_| {
        steel::SteelErr::new(
            steel::rerrs::ErrorKind::ConversionError,
            format!("expected non-negative integer for usize, found: {n}"),
        )
    })
}

pub fn steel_to_u8(val: &SteelVal) -> Result<u8> {
    let n = steel_to_i64(val)?;
    u8::try_from(n).map_err(|_| {
        steel::SteelErr::new(
            steel::rerrs::ErrorKind::ConversionError,
            format!("expected byte integer (0..=255), found: {n}"),
        )
    })
}

pub fn steel_to_u32(val: &SteelVal) -> Result<u32> {
    let n = steel_to_i64(val)?;
    u32::try_from(n).map_err(|_| {
        steel::SteelErr::new(
            steel::rerrs::ErrorKind::ConversionError,
            format!("expected u32 integer, found: {n}"),
        )
    })
}

pub fn steel_to_bool(val: &SteelVal) -> Result<bool> {
    match val {
        SteelVal::BoolV(b) => Ok(*b),
        _ => stop!(TypeMismatch => "expected boolean (#t or #f), found: {}", val),
    }
}

pub fn steel_to_symbol_or_str(val: &SteelVal) -> Result<String> {
    match val {
        SteelVal::SymbolV(s) | SteelVal::StringV(s) => Ok(s.as_str().to_owned()),
        _ => stop!(TypeMismatch => "expected symbol or string, found: {}", val),
    }
}

pub fn steel_to_optional_f32(val: &SteelVal) -> Result<Option<f32>> {
    match val {
        SteelVal::BoolV(false) | SteelVal::Void => Ok(None),
        other => Ok(Some(steel_to_f32(other)?)),
    }
}

fn sequence_elements(val: &SteelVal) -> Option<Vec<SteelVal>> {
    match val {
        SteelVal::ListV(list) => Some(list.iter().cloned().collect()),
        SteelVal::VectorV(vec) => Some(vec.iter().cloned().collect()),
        SteelVal::MutableVector(vec) => Some(vec.borrow(|inner| inner.clone())),
        _ => None,
    }
}

pub fn steel_to_vec2(val: &SteelVal) -> Result<Vec2> {
    if let Ok(v) = MqVec2::from_steelval(val) {
        return Ok(v.0);
    }
    if let Some(elems) = sequence_elements(val) {
        if elems.len() == 2 {
            return Ok(Vec2::new(
                steel_to_f32(&elems[0])?,
                steel_to_f32(&elems[1])?,
            ));
        }
        stop!(ConversionError => "expected vec2 or 2-element list/vector, got {} elements", elems.len());
    }
    stop!(TypeMismatch => "expected vec2 or 2-element list/vector, found: {}", val)
}

pub fn steel_to_vec3(val: &SteelVal) -> Result<Vec3> {
    if let Ok(v) = MqVec3::from_steelval(val) {
        return Ok(v.0);
    }
    if let Some(elems) = sequence_elements(val) {
        if elems.len() == 3 {
            return Ok(Vec3::new(
                steel_to_f32(&elems[0])?,
                steel_to_f32(&elems[1])?,
                steel_to_f32(&elems[2])?,
            ));
        }
        stop!(ConversionError => "expected vec3 or 3-element list/vector, got {} elements", elems.len());
    }
    stop!(TypeMismatch => "expected vec3 or 3-element list/vector, found: {}", val)
}

pub fn steel_to_color(val: &SteelVal) -> Result<Color> {
    if let Ok(c) = MqColor::from_steelval(val) {
        return Ok(c.0);
    }
    if let Some(elems) = sequence_elements(val) {
        match elems.len() {
            3 => {
                return Ok(Color::new(
                    steel_to_f32(&elems[0])?,
                    steel_to_f32(&elems[1])?,
                    steel_to_f32(&elems[2])?,
                    1.0,
                ));
            }
            4 => {
                return Ok(Color::new(
                    steel_to_f32(&elems[0])?,
                    steel_to_f32(&elems[1])?,
                    steel_to_f32(&elems[2])?,
                    steel_to_f32(&elems[3])?,
                ));
            }
            len => {
                stop!(ConversionError => "expected color or 3/4-element list/vector, got {} elements", len);
            }
        }
    }
    stop!(TypeMismatch => "expected color or 3/4-element list/vector, found: {}", val)
}

pub fn vec2_into_steelval(v: Vec2) -> Result<SteelVal> {
    MqVec2(v).into_steelval()
}

pub fn vec3_into_steelval(v: Vec3) -> Result<SteelVal> {
    MqVec3(v).into_steelval()
}

pub fn color_into_steelval(c: Color) -> Result<SteelVal> {
    MqColor(c).into_steelval()
}

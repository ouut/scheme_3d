use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use macroquad::color::Color;
use macroquad::math::{Vec2, Vec3};
use steel::rvals::{FromSteelVal, IntoSteelVal, Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel::stop;

use crate::types::{
    color_into_steelval, steel_to_f64, steel_to_symbol_or_str, vec2_into_steelval,
    vec3_into_steelval, MqCamera2D, MqCamera3D, MqColor, MqVec2, MqVec3,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PersistValue {
    Void,
    Bool(bool),
    Int(isize),
    Float(f64),
    String(String),
    Symbol(String),
    List(Vec<PersistValue>),
    Vec(Vec<PersistValue>),
    Map(Vec<(PersistValue, PersistValue)>),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Color([f32; 4]),
    Camera2D(MqCamera2D),
    Camera3D(MqCamera3D),
}

#[derive(Clone, Default, Debug)]
pub struct PersistStore {
    inner: Arc<Mutex<HashMap<String, PersistValue>>>,
}

impl PersistStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    pub fn get(&self, key: &str) -> Option<PersistValue> {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(key)
            .cloned()
    }

    pub fn set(&self, key: impl Into<String>, value: PersistValue) {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(key.into(), value);
    }

    pub fn contains(&self, key: &str) -> bool {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .contains_key(key)
    }

    pub fn remove(&self, key: &str) -> Option<PersistValue> {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(key)
    }

    pub fn clear(&self) {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
    }
}

pub fn steelval_to_persist(val: &SteelVal) -> Result<PersistValue> {
    if let Ok(v) = MqVec2::from_steelval(val) {
        return Ok(PersistValue::Vec2([v.0.x, v.0.y]));
    }
    if let Ok(v) = MqVec3::from_steelval(val) {
        return Ok(PersistValue::Vec3([v.0.x, v.0.y, v.0.z]));
    }
    if let Ok(c) = MqColor::from_steelval(val) {
        return Ok(PersistValue::Color([c.0.r, c.0.g, c.0.b, c.0.a]));
    }
    if let Ok(cam) = MqCamera2D::from_steelval(val) {
        return Ok(PersistValue::Camera2D(cam));
    }
    if let Ok(cam) = MqCamera3D::from_steelval(val) {
        return Ok(PersistValue::Camera3D(cam));
    }

    match val {
        SteelVal::Void => Ok(PersistValue::Void),
        SteelVal::BoolV(b) => Ok(PersistValue::Bool(*b)),
        SteelVal::IntV(i) => Ok(PersistValue::Int(*i)),
        SteelVal::NumV(n) => Ok(PersistValue::Float(*n)),
        SteelVal::Rational(_) | SteelVal::BigNum(_) | SteelVal::BigRational(_) => {
            Ok(PersistValue::Float(steel_to_f64(val)?))
        }
        SteelVal::StringV(s) => Ok(PersistValue::String(s.as_str().to_owned())),
        SteelVal::SymbolV(s) => Ok(PersistValue::Symbol(s.as_str().to_owned())),
        SteelVal::ListV(list) => {
            let items: Result<Vec<PersistValue>> = list.iter().map(steelval_to_persist).collect();
            Ok(PersistValue::List(items?))
        }
        SteelVal::VectorV(vec) => {
            let items: Result<Vec<PersistValue>> = vec.iter().map(steelval_to_persist).collect();
            Ok(PersistValue::Vec(items?))
        }
        SteelVal::MutableVector(vec) => {
            let snapshot = vec.borrow(|inner| inner.clone());
            let items: Result<Vec<PersistValue>> =
                snapshot.iter().map(steelval_to_persist).collect();
            Ok(PersistValue::Vec(items?))
        }
        SteelVal::HashMapV(hm) => {
            let mut pairs = Vec::with_capacity(hm.len());
            for (k, v) in hm.iter() {
                pairs.push((steelval_to_persist(k)?, steelval_to_persist(v)?));
            }
            Ok(PersistValue::Map(pairs))
        }
        other => stop!(
            ConversionError => "state-set!: unsupported value for cross-reload persistence: {}",
            other
        ),
    }
}

#[allow(clippy::mutable_key_type)]
pub fn persist_to_steelval(val: &PersistValue) -> Result<SteelVal> {
    match val {
        PersistValue::Void => Ok(SteelVal::Void),
        PersistValue::Bool(b) => Ok(SteelVal::BoolV(*b)),
        PersistValue::Int(i) => Ok(SteelVal::IntV(*i)),
        PersistValue::Float(n) => Ok(SteelVal::NumV(*n)),
        PersistValue::String(s) => Ok(SteelVal::StringV(s.clone().into())),
        PersistValue::Symbol(s) => Ok(SteelVal::SymbolV(s.clone().into())),
        PersistValue::List(items) => {
            let converted: Result<Vec<SteelVal>> = items.iter().map(persist_to_steelval).collect();
            Ok(SteelVal::ListV(converted?.into()))
        }
        PersistValue::Vec(items) => {
            let converted: Result<Vec<SteelVal>> = items.iter().map(persist_to_steelval).collect();
            converted?.into_steelval()
        }
        PersistValue::Map(pairs) => {
            let mut map: HashMap<SteelVal, SteelVal> = HashMap::with_capacity(pairs.len());
            for (k, v) in pairs {
                map.insert(persist_to_steelval(k)?, persist_to_steelval(v)?);
            }
            map.into_steelval()
        }
        PersistValue::Vec2([x, y]) => vec2_into_steelval(Vec2::new(*x, *y)),
        PersistValue::Vec3([x, y, z]) => vec3_into_steelval(Vec3::new(*x, *y, *z)),
        PersistValue::Color([r, g, b, a]) => color_into_steelval(Color::new(*r, *g, *b, *a)),
        PersistValue::Camera2D(cam) => (*cam).into_steelval(),
        PersistValue::Camera3D(cam) => (*cam).into_steelval(),
    }
}

pub fn register(module: &mut BuiltInModule, store: &PersistStore) {
    {
        let store = store.clone();
        module.register_fn(
            "state-get",
            move |key: SteelVal, default: SteelVal| -> Result<SteelVal> {
                let k = steel_to_symbol_or_str(&key)?;
                match store.get(&k) {
                    Some(val) => persist_to_steelval(&val),
                    None => Ok(default),
                }
            },
        );
    }
    {
        let store = store.clone();
        module.register_fn("state-try-get", move |key: SteelVal| -> Result<SteelVal> {
            let k = steel_to_symbol_or_str(&key)?;
            match store.get(&k) {
                Some(val) => persist_to_steelval(&val),
                None => Ok(SteelVal::BoolV(false)),
            }
        });
    }
    {
        let store = store.clone();
        module.register_fn(
            "state-set!",
            move |key: SteelVal, val: SteelVal| -> Result<SteelVal> {
                let k = steel_to_symbol_or_str(&key)?;
                let pv = steelval_to_persist(&val)?;
                store.set(k, pv);
                Ok(val)
            },
        );
    }
    {
        let store = store.clone();
        module.register_fn("state-has?", move |key: SteelVal| -> Result<bool> {
            let k = steel_to_symbol_or_str(&key)?;
            Ok(store.contains(&k))
        });
    }
    {
        let store = store.clone();
        module.register_fn("state-remove!", move |key: SteelVal| -> Result<bool> {
            let k = steel_to_symbol_or_str(&key)?;
            Ok(store.remove(&k).is_some())
        });
    }
    {
        let store = store.clone();
        module.register_fn("state-clear!", move || {
            store.clear();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::{camera, color, math};
    use crate::types::register_types;
    use steel::steel_vm::engine::Engine;

    fn make_vm(store: &PersistStore) -> Engine {
        let mut vm = Engine::new();
        let mut module = BuiltInModule::new("macroquad");
        register_types(&mut module);
        math::register(&mut module);
        color::register(&mut module);
        camera::register(&mut module);
        register(&mut module, store);
        vm.register_module(module);
        vm.run("(require-builtin macroquad)").unwrap();
        vm
    }

    #[test]
    fn test_persist_across_separate_engines() {
        let store = PersistStore::new();
        {
            let mut vm1 = make_vm(&store);
            vm1.run(
                r#"
                (state-set! 'score 42)
                (state-set! "pos" (vec3 1 2 3))
                (state-set! 'cam (make-camera-3d (vec3 4 5 6) (vec3 0 0 0) (vec3 0 1 0) 1.0))
                (state-set! 'items (list 'a "b" #(10 20)))
                "#,
            )
            .unwrap();
        }
        assert_eq!(store.len(), 4);

        let mut vm2 = make_vm(&store);
        let res = vm2
            .run(
                r#"
                (list
                    (state-has? 'score)
                    (state-get "score" 0)
                    (state-get 'missing 99)
                    (state-try-get 'missing)
                    (state-get 'pos (vec3 0 0 0))
                    (camera-3d-position (state-get 'cam (camera-3d-default)))
                    (state-remove! 'score)
                    (state-has? 'score))
                "#,
            )
            .unwrap();
        let SteelVal::ListV(items) = &res[0] else {
            panic!("expected list");
        };
        let v: Vec<SteelVal> = items.iter().cloned().collect();
        assert_eq!(v[0], SteelVal::BoolV(true));
        assert_eq!(v[1], SteelVal::IntV(42));
        assert_eq!(v[2], SteelVal::IntV(99));
        assert_eq!(v[3], SteelVal::BoolV(false));
        assert_eq!(
            MqVec3::from_steelval(&v[4]).unwrap(),
            MqVec3(Vec3::new(1.0, 2.0, 3.0))
        );
        assert_eq!(
            MqVec3::from_steelval(&v[5]).unwrap(),
            MqVec3(Vec3::new(4.0, 5.0, 6.0))
        );
        assert_eq!(v[6], SteelVal::BoolV(true));
        assert_eq!(v[7], SteelVal::BoolV(false));
    }
}

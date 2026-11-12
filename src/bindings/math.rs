use macroquad::math::{Vec2, Vec3};
use steel::rvals::{Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel::stop;

use crate::types::{
    steel_to_f32, steel_to_f64, steel_to_vec2, steel_to_vec3, vec2_into_steelval,
    vec3_into_steelval, MqVec2, MqVec3,
};

pub fn register(module: &mut BuiltInModule) {
    module.register_value("PI", SteelVal::NumV(std::f64::consts::PI));
    module.register_value("TAU", SteelVal::NumV(std::f64::consts::TAU));

    module
        .register_fn("vec2", |x: SteelVal, y: SteelVal| -> Result<SteelVal> {
            vec2_into_steelval(Vec2::new(steel_to_f32(&x)?, steel_to_f32(&y)?))
        })
        .register_fn("vec2-x", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec2(&v)?.x as f64)
        })
        .register_fn("vec2-y", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec2(&v)?.y as f64)
        })
        .register_fn("vec2+", |a: SteelVal, b: SteelVal| -> Result<SteelVal> {
            vec2_into_steelval(steel_to_vec2(&a)? + steel_to_vec2(&b)?)
        })
        .register_fn("vec2-", |a: SteelVal, b: SteelVal| -> Result<SteelVal> {
            vec2_into_steelval(steel_to_vec2(&a)? - steel_to_vec2(&b)?)
        })
        .register_fn("vec2*", |a: SteelVal, b: SteelVal| -> Result<SteelVal> {
            if let Ok(va) = steel_to_vec2(&a) {
                if let Ok(vb) = steel_to_vec2(&b) {
                    return vec2_into_steelval(va * vb);
                }
                let sb = steel_to_f32(&b)?;
                return vec2_into_steelval(va * sb);
            }
            let sa = steel_to_f32(&a)?;
            let vb = steel_to_vec2(&b)?;
            vec2_into_steelval(vb * sa)
        })
        .register_fn("vec2-length", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec2(&v)?.length() as f64)
        })
        .register_fn("vec2-length-squared", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec2(&v)?.length_squared() as f64)
        })
        .register_fn("vec2-normalize", |v: SteelVal| -> Result<SteelVal> {
            vec2_into_steelval(steel_to_vec2(&v)?.normalize_or_zero())
        })
        .register_fn("vec2-dot", |a: SteelVal, b: SteelVal| -> Result<f64> {
            Ok(steel_to_vec2(&a)?.dot(steel_to_vec2(&b)?) as f64)
        })
        .register_fn(
            "vec2-lerp",
            |a: SteelVal, b: SteelVal, t: SteelVal| -> Result<SteelVal> {
                vec2_into_steelval(steel_to_vec2(&a)?.lerp(steel_to_vec2(&b)?, steel_to_f32(&t)?))
            },
        )
        .register_fn("vec2-distance", |a: SteelVal, b: SteelVal| -> Result<f64> {
            Ok(steel_to_vec2(&a)?.distance(steel_to_vec2(&b)?) as f64)
        });

    module
        .register_fn(
            "vec3",
            |x: SteelVal, y: SteelVal, z: SteelVal| -> Result<SteelVal> {
                vec3_into_steelval(Vec3::new(
                    steel_to_f32(&x)?,
                    steel_to_f32(&y)?,
                    steel_to_f32(&z)?,
                ))
            },
        )
        .register_fn("vec3-x", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec3(&v)?.x as f64)
        })
        .register_fn("vec3-y", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec3(&v)?.y as f64)
        })
        .register_fn("vec3-z", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec3(&v)?.z as f64)
        })
        .register_fn("vec3+", |a: SteelVal, b: SteelVal| -> Result<SteelVal> {
            vec3_into_steelval(steel_to_vec3(&a)? + steel_to_vec3(&b)?)
        })
        .register_fn("vec3-", |a: SteelVal, b: SteelVal| -> Result<SteelVal> {
            vec3_into_steelval(steel_to_vec3(&a)? - steel_to_vec3(&b)?)
        })
        .register_fn("vec3*", |a: SteelVal, b: SteelVal| -> Result<SteelVal> {
            if let Ok(va) = steel_to_vec3(&a) {
                if let Ok(vb) = steel_to_vec3(&b) {
                    return vec3_into_steelval(va * vb);
                }
                let sb = steel_to_f32(&b)?;
                return vec3_into_steelval(va * sb);
            }
            let sa = steel_to_f32(&a)?;
            let vb = steel_to_vec3(&b)?;
            vec3_into_steelval(vb * sa)
        })
        .register_fn("vec3-length", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec3(&v)?.length() as f64)
        })
        .register_fn("vec3-length-squared", |v: SteelVal| -> Result<f64> {
            Ok(steel_to_vec3(&v)?.length_squared() as f64)
        })
        .register_fn("vec3-normalize", |v: SteelVal| -> Result<SteelVal> {
            vec3_into_steelval(steel_to_vec3(&v)?.normalize_or_zero())
        })
        .register_fn("vec3-dot", |a: SteelVal, b: SteelVal| -> Result<f64> {
            Ok(steel_to_vec3(&a)?.dot(steel_to_vec3(&b)?) as f64)
        })
        .register_fn(
            "vec3-cross",
            |a: SteelVal, b: SteelVal| -> Result<SteelVal> {
                vec3_into_steelval(steel_to_vec3(&a)?.cross(steel_to_vec3(&b)?))
            },
        )
        .register_fn(
            "vec3-lerp",
            |a: SteelVal, b: SteelVal, t: SteelVal| -> Result<SteelVal> {
                vec3_into_steelval(steel_to_vec3(&a)?.lerp(steel_to_vec3(&b)?, steel_to_f32(&t)?))
            },
        )
        .register_fn("vec3-distance", |a: SteelVal, b: SteelVal| -> Result<f64> {
            Ok(steel_to_vec3(&a)?.distance(steel_to_vec3(&b)?) as f64)
        });

    module
        .register_fn(
            "clamp",
            |x: SteelVal, min: SteelVal, max: SteelVal| -> Result<f64> {
                let x = steel_to_f64(&x)?;
                let min = steel_to_f64(&min)?;
                let max = steel_to_f64(&max)?;
                if min > max {
                    stop!(Generic => "clamp: min ({}) must be <= max ({})", min, max);
                }
                Ok(x.clamp(min, max))
            },
        )
        .register_fn(
            "lerp",
            |a: SteelVal, b: SteelVal, t: SteelVal| -> Result<f64> {
                let a = steel_to_f64(&a)?;
                let b = steel_to_f64(&b)?;
                let t = steel_to_f64(&t)?;
                Ok(a + (b - a) * t)
            },
        )
        .register_fn("deg->rad", |deg: SteelVal| -> Result<f64> {
            Ok(steel_to_f64(&deg)?.to_radians())
        })
        .register_fn("rad->deg", |rad: SteelVal| -> Result<f64> {
            Ok(steel_to_f64(&rad)?.to_degrees())
        });

    let _ = (MqVec2(Vec2::ZERO), MqVec3(Vec3::ZERO));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{register_types, FromSteelVal};
    use steel::steel_vm::engine::Engine;

    fn test_engine() -> Engine {
        let mut engine = Engine::new();
        let mut module = BuiltInModule::new("macroquad");
        register_types(&mut module);
        register(&mut module);
        engine.register_module(module);
        engine.run("(require-builtin macroquad)").unwrap();
        engine
    }

    #[test]
    fn test_vec2_operations_and_coercion() {
        let mut vm = test_engine();
        let res = vm
            .run(
                r#"
                (define a (vec2 3 4.0))
                (define b (vec2 1 2))
                (list
                    (vec2? a)
                    (vec2-x a)
                    (vec2-y a)
                    (vec2-length a)
                    (vec2-length-squared a)
                    (vec2+ a '(1 2))
                    (vec2- a b)
                    (vec2* a 2)
                    (vec2* 0.5 a)
                    (vec2* a #(2 3))
                    (vec2-dot a b)
                    (vec2-distance a (vec2 0 0))
                    (vec2-lerp (vec2 0 0) (vec2 10 20) 0.5)
                    (vec2-normalize (vec2 0 0))
                    (equal? (vec2 1 2) (vec2 1.0 2.0))
                    (equal? (vec2 1 2) (vec2 1 3)))
                "#,
            )
            .unwrap();

        let SteelVal::ListV(items) = res.last().unwrap() else {
            panic!("expected list");
        };
        let v: Vec<SteelVal> = items.iter().cloned().collect();
        assert_eq!(v[0], SteelVal::BoolV(true));
        assert_eq!(v[1], SteelVal::NumV(3.0));
        assert_eq!(v[2], SteelVal::NumV(4.0));
        assert_eq!(v[3], SteelVal::NumV(5.0));
        assert_eq!(v[4], SteelVal::NumV(25.0));
        assert_eq!(MqVec2::from_steelval(&v[5]).unwrap(), MqVec2(Vec2::new(4.0, 6.0)));
        assert_eq!(MqVec2::from_steelval(&v[6]).unwrap(), MqVec2(Vec2::new(2.0, 2.0)));
        assert_eq!(MqVec2::from_steelval(&v[7]).unwrap(), MqVec2(Vec2::new(6.0, 8.0)));
        assert_eq!(MqVec2::from_steelval(&v[8]).unwrap(), MqVec2(Vec2::new(1.5, 2.0)));
        assert_eq!(MqVec2::from_steelval(&v[9]).unwrap(), MqVec2(Vec2::new(6.0, 12.0)));
        assert_eq!(v[10], SteelVal::NumV(11.0));
        assert_eq!(v[11], SteelVal::NumV(5.0));
        assert_eq!(MqVec2::from_steelval(&v[12]).unwrap(), MqVec2(Vec2::new(5.0, 10.0)));
        assert_eq!(MqVec2::from_steelval(&v[13]).unwrap(), MqVec2(Vec2::ZERO));
        assert_eq!(v[14], SteelVal::BoolV(true));
        assert_eq!(v[15], SteelVal::BoolV(false));
    }

    #[test]
    fn test_vec3_and_scalars() {
        let mut vm = test_engine();
        let res = vm
            .run(
                r#"
                (define x (vec3 1 0 0))
                (define y (vec3 0 1 0))
                (list
                    (vec3? x)
                    (vec3-cross x y)
                    (vec3+ '(1 2 3) #(4 5 6))
                    (vec3* 3 (vec3 1 2 3))
                    (vec3-distance (vec3 0 0 0) (vec3 0 3 4))
                    (clamp 15 0 10)
                    (lerp 10 20 0.25)
                    (< (abs (- (deg->rad 180) PI)) 0.0001)
                    (< (abs (- (rad->deg PI) 180.0)) 0.0001))
                "#,
            )
            .unwrap();
        let SteelVal::ListV(items) = res.last().unwrap() else {
            panic!("expected list");
        };
        let v: Vec<SteelVal> = items.iter().cloned().collect();
        assert_eq!(v[0], SteelVal::BoolV(true));
        assert_eq!(MqVec3::from_steelval(&v[1]).unwrap(), MqVec3(Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(MqVec3::from_steelval(&v[2]).unwrap(), MqVec3(Vec3::new(5.0, 7.0, 9.0)));
        assert_eq!(MqVec3::from_steelval(&v[3]).unwrap(), MqVec3(Vec3::new(3.0, 6.0, 9.0)));
        assert_eq!(v[4], SteelVal::NumV(5.0));
        assert_eq!(v[5], SteelVal::NumV(10.0));
        assert_eq!(v[6], SteelVal::NumV(12.5));
        assert_eq!(v[7], SteelVal::BoolV(true));
        assert_eq!(v[8], SteelVal::BoolV(true));
    }
}

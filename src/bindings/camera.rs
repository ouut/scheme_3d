use macroquad::camera::{pop_camera_state, push_camera_state, set_camera, set_default_camera};
use steel::rvals::{AsRefMutSteelVal, AsRefSteelVal, IntoSteelVal, Result, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;
use steel::stop;

use crate::types::{
    steel_to_f32, steel_to_optional_f32, steel_to_symbol_or_str, steel_to_vec2, steel_to_vec3,
    vec2_into_steelval, vec3_into_steelval, MqCamera2D, MqCamera3D, MqProjection,
};

fn parse_projection(val: &SteelVal) -> Result<MqProjection> {
    let s = steel_to_symbol_or_str(val)?.to_ascii_lowercase();
    match s.as_str() {
        "perspective" => Ok(MqProjection::Perspective),
        "orthographic" | "orthographics" | "ortho" => Ok(MqProjection::Orthographic),
        other => stop!(
            Generic => "camera-3d-set-projection!: expected 'perspective or 'orthographic, got {}",
            other
        ),
    }
}

pub fn register(module: &mut BuiltInModule) {
    module
        .register_fn(
            "make-camera-3d",
            |pos: SteelVal, target: SteelVal, up: SteelVal, fovy: SteelVal| -> Result<SteelVal> {
                let cam = MqCamera3D {
                    position: steel_to_vec3(&pos)?,
                    target: steel_to_vec3(&target)?,
                    up: steel_to_vec3(&up)?,
                    fovy: steel_to_f32(&fovy)?,
                    ..MqCamera3D::default()
                };
                cam.into_steelval()
            },
        )
        .register_fn("camera-3d-default", || -> Result<SteelVal> {
            MqCamera3D::default().into_steelval()
        })
        .register_fn("camera-3d-position", |cam: SteelVal| -> Result<SteelVal> {
            let cam = MqCamera3D::as_ref(&cam)?;
            vec3_into_steelval(cam.position)
        })
        .register_fn("camera-3d-target", |cam: SteelVal| -> Result<SteelVal> {
            let cam = MqCamera3D::as_ref(&cam)?;
            vec3_into_steelval(cam.target)
        })
        .register_fn("camera-3d-up", |cam: SteelVal| -> Result<SteelVal> {
            let cam = MqCamera3D::as_ref(&cam)?;
            vec3_into_steelval(cam.up)
        })
        .register_fn("camera-3d-fovy", |cam: SteelVal| -> Result<f64> {
            let cam = MqCamera3D::as_ref(&cam)?;
            Ok(cam.fovy as f64)
        })
        .register_fn("camera-3d-projection", |cam: SteelVal| -> Result<SteelVal> {
            let cam = MqCamera3D::as_ref(&cam)?;
            let sym = match cam.projection {
                MqProjection::Perspective => "perspective",
                MqProjection::Orthographic => "orthographic",
            };
            Ok(SteelVal::SymbolV(sym.into()))
        })
        .register_fn("camera-3d-near", |cam: SteelVal| -> Result<f64> {
            let cam = MqCamera3D::as_ref(&cam)?;
            Ok(cam.z_near as f64)
        })
        .register_fn("camera-3d-far", |cam: SteelVal| -> Result<f64> {
            let cam = MqCamera3D::as_ref(&cam)?;
            Ok(cam.z_far as f64)
        })
        .register_fn(
            "camera-3d-set-position!",
            |cam: SteelVal, pos: SteelVal| -> Result<()> {
                let new_pos = steel_to_vec3(&pos)?;
                let mut cam = MqCamera3D::as_mut_ref(&cam)?;
                cam.position = new_pos;
                Ok(())
            },
        )
        .register_fn(
            "camera-3d-set-target!",
            |cam: SteelVal, target: SteelVal| -> Result<()> {
                let new_target = steel_to_vec3(&target)?;
                let mut cam = MqCamera3D::as_mut_ref(&cam)?;
                cam.target = new_target;
                Ok(())
            },
        )
        .register_fn(
            "camera-3d-set-up!",
            |cam: SteelVal, up: SteelVal| -> Result<()> {
                let new_up = steel_to_vec3(&up)?;
                let mut cam = MqCamera3D::as_mut_ref(&cam)?;
                cam.up = new_up;
                Ok(())
            },
        )
        .register_fn(
            "camera-3d-set-fovy!",
            |cam: SteelVal, fovy: SteelVal| -> Result<()> {
                let new_fovy = steel_to_f32(&fovy)?;
                let mut cam = MqCamera3D::as_mut_ref(&cam)?;
                cam.fovy = new_fovy;
                Ok(())
            },
        )
        .register_fn(
            "camera-3d-set-aspect!",
            |cam: SteelVal, aspect: SteelVal| -> Result<()> {
                let new_aspect = steel_to_optional_f32(&aspect)?;
                let mut cam = MqCamera3D::as_mut_ref(&cam)?;
                cam.aspect = new_aspect;
                Ok(())
            },
        )
        .register_fn(
            "camera-3d-set-projection!",
            |cam: SteelVal, proj: SteelVal| -> Result<()> {
                let new_proj = parse_projection(&proj)?;
                let mut cam = MqCamera3D::as_mut_ref(&cam)?;
                cam.projection = new_proj;
                Ok(())
            },
        )
        .register_fn(
            "camera-3d-set-clip!",
            |cam: SteelVal, near: SteelVal, far: SteelVal| -> Result<()> {
                let z_near = steel_to_f32(&near)?;
                let z_far = steel_to_f32(&far)?;
                if z_near >= z_far {
                    stop!(
                        Generic => "camera-3d-set-clip!: near ({}) must be < far ({})",
                        z_near,
                        z_far
                    );
                }
                let mut cam = MqCamera3D::as_mut_ref(&cam)?;
                cam.z_near = z_near;
                cam.z_far = z_far;
                Ok(())
            },
        );

    module
        .register_fn(
            "make-camera-2d",
            |target: SteelVal,
             zoom: SteelVal,
             rotation: SteelVal,
             offset: SteelVal|
             -> Result<SteelVal> {
                let cam = MqCamera2D {
                    target: steel_to_vec2(&target)?,
                    zoom: steel_to_vec2(&zoom)?,
                    rotation: steel_to_f32(&rotation)?,
                    offset: steel_to_vec2(&offset)?,
                    viewport: None,
                };
                cam.into_steelval()
            },
        )
        .register_fn("camera-2d-default", || -> Result<SteelVal> {
            MqCamera2D::default().into_steelval()
        })
        .register_fn("camera-2d-target", |cam: SteelVal| -> Result<SteelVal> {
            let cam = MqCamera2D::as_ref(&cam)?;
            vec2_into_steelval(cam.target)
        })
        .register_fn("camera-2d-zoom", |cam: SteelVal| -> Result<SteelVal> {
            let cam = MqCamera2D::as_ref(&cam)?;
            vec2_into_steelval(cam.zoom)
        })
        .register_fn("camera-2d-rotation", |cam: SteelVal| -> Result<f64> {
            let cam = MqCamera2D::as_ref(&cam)?;
            Ok(cam.rotation as f64)
        })
        .register_fn("camera-2d-offset", |cam: SteelVal| -> Result<SteelVal> {
            let cam = MqCamera2D::as_ref(&cam)?;
            vec2_into_steelval(cam.offset)
        })
        .register_fn(
            "camera-2d-set-target!",
            |cam: SteelVal, target: SteelVal| -> Result<()> {
                let v = steel_to_vec2(&target)?;
                let mut cam = MqCamera2D::as_mut_ref(&cam)?;
                cam.target = v;
                Ok(())
            },
        )
        .register_fn(
            "camera-2d-set-zoom!",
            |cam: SteelVal, zoom: SteelVal| -> Result<()> {
                let v = steel_to_vec2(&zoom)?;
                let mut cam = MqCamera2D::as_mut_ref(&cam)?;
                cam.zoom = v;
                Ok(())
            },
        )
        .register_fn(
            "camera-2d-set-rotation!",
            |cam: SteelVal, rot: SteelVal| -> Result<()> {
                let r = steel_to_f32(&rot)?;
                let mut cam = MqCamera2D::as_mut_ref(&cam)?;
                cam.rotation = r;
                Ok(())
            },
        )
        .register_fn(
            "camera-2d-set-offset!",
            |cam: SteelVal, offset: SteelVal| -> Result<()> {
                let v = steel_to_vec2(&offset)?;
                let mut cam = MqCamera2D::as_mut_ref(&cam)?;
                cam.offset = v;
                Ok(())
            },
        )
        .register_fn(
            "camera-2d-world->screen",
            |cam: SteelVal, point: SteelVal| -> Result<SteelVal> {
                let cam = MqCamera2D::as_ref(&cam)?;
                let p = steel_to_vec2(&point)?;
                vec2_into_steelval(cam.to_macroquad().world_to_screen(p))
            },
        )
        .register_fn(
            "camera-2d-screen->world",
            |cam: SteelVal, point: SteelVal| -> Result<SteelVal> {
                let cam = MqCamera2D::as_ref(&cam)?;
                let p = steel_to_vec2(&point)?;
                vec2_into_steelval(cam.to_macroquad().screen_to_world(p))
            },
        );

    module
        .register_fn("set-camera-3d", |cam: SteelVal| -> Result<()> {
            let cam = MqCamera3D::as_ref(&cam)?;
            let mq = cam.to_macroquad();
            set_camera(&mq);
            Ok(())
        })
        .register_fn("set-camera-2d", |cam: SteelVal| -> Result<()> {
            let cam = MqCamera2D::as_ref(&cam)?;
            let mq = cam.to_macroquad();
            set_camera(&mq);
            Ok(())
        })
        .register_fn("set-default-camera", || -> Result<()> {
            set_default_camera();
            Ok(())
        })
        .register_fn("push-camera-state", || -> Result<()> {
            push_camera_state();
            Ok(())
        })
        .register_fn("pop-camera-state", || -> Result<()> {
            pop_camera_state();
            Ok(())
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::math;
    use crate::types::{register_types, FromSteelVal, MqVec2, MqVec3};
    use macroquad::math::{Vec2, Vec3};
    use std::sync::{Arc, Mutex};
    use steel::steel_vm::engine::Engine;

    fn test_engine() -> Engine {
        let mut vm = Engine::new();
        let mut module = BuiltInModule::new("macroquad");
        register_types(&mut module);
        math::register(&mut module);
        register(&mut module);
        vm.register_module(module);
        vm.run("(require-builtin macroquad)").unwrap();
        vm
    }

    #[test]
    fn test_camera_3d_creation_and_mutation() {
        let mut vm = test_engine();
        let res = vm
            .run(
                r#"
                (define cam (make-camera-3d (vec3 0 5 10) (vec3 0 0 0) (vec3 0 1 0) (deg->rad 60)))
                (camera-3d-set-position! cam (vec3 3 6 9))
                (camera-3d-set-target! cam '(1 2 3))
                (camera-3d-set-projection! cam 'orthographic)
                (camera-3d-set-clip! cam 0.5 500.0)
                (camera-3d-set-aspect! cam 1.777)
                (camera-3d-set-aspect! cam #f)
                (list
                    (camera-3d? cam)
                    (camera-3d-position cam)
                    (camera-3d-target cam)
                    (camera-3d-projection cam)
                    (camera-3d-near cam)
                    (camera-3d-far cam))
                "#,
            )
            .unwrap();
        let SteelVal::ListV(items) = res.last().unwrap() else {
            panic!("expected list");
        };
        let v: Vec<SteelVal> = items.iter().cloned().collect();
        assert_eq!(v[0], SteelVal::BoolV(true));
        assert_eq!(
            MqVec3::from_steelval(&v[1]).unwrap(),
            MqVec3(Vec3::new(3.0, 6.0, 9.0))
        );
        assert_eq!(
            MqVec3::from_steelval(&v[2]).unwrap(),
            MqVec3(Vec3::new(1.0, 2.0, 3.0))
        );
        assert_eq!(v[3], SteelVal::SymbolV("orthographic".into()));
        assert_eq!(v[4], SteelVal::NumV(0.5));
        assert_eq!(v[5], SteelVal::NumV(500.0));

        assert!(vm.run("(camera-3d-set-projection! cam 'invalid)").is_err());
        assert!(vm.run("(camera-3d-set-clip! cam 100 10)").is_err());
    }

    #[test]
    fn test_camera_2d_creation_and_mutation() {
        let mut vm = test_engine();
        let res = vm
            .run(
                r#"
                (define c2 (make-camera-2d (vec2 10 20) (vec2 1 1) 0 (vec2 0 0)))
                (camera-2d-set-target! c2 (vec2 50 60))
                (camera-2d-set-rotation! c2 45)
                (list
                    (camera-2d? c2)
                    (camera-2d-target c2)
                    (camera-2d-rotation c2))
                "#,
            )
            .unwrap();
        let SteelVal::ListV(items) = res.last().unwrap() else {
            panic!("expected list");
        };
        let v: Vec<SteelVal> = items.iter().cloned().collect();
        assert_eq!(v[0], SteelVal::BoolV(true));
        assert_eq!(
            MqVec2::from_steelval(&v[1]).unwrap(),
            MqVec2(Vec2::new(50.0, 60.0))
        );
        assert_eq!(v[2], SteelVal::NumV(45.0));
    }

    #[test]
    fn test_prelude_with_camera_exception_safety() {
        let trace: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
        let mut vm = Engine::new();
        let mut module = BuiltInModule::new("macroquad");
        register_types(&mut module);
        register(&mut module);

        {
            let t = Arc::clone(&trace);
            module.register_fn("push-camera-state", move || {
                t.lock().unwrap().push("push");
            });
        }
        {
            let t = Arc::clone(&trace);
            module.register_fn("set-camera-3d", move |_cam: SteelVal| {
                t.lock().unwrap().push("set3d");
            });
        }
        {
            let t = Arc::clone(&trace);
            module.register_fn("pop-camera-state", move || {
                t.lock().unwrap().push("pop");
            });
        }

        vm.register_module(module);
        vm.register_steel_module(
            "scheme3d/prelude".to_string(),
            include_str!("../../scheme/prelude.scm").to_string(),
        );
        vm.run("(require-builtin macroquad)\n(require \"scheme3d/prelude\")")
            .unwrap();

        let ok_res = vm
            .run("(with-camera-3d (camera-3d-default) (lambda () 42))")
            .unwrap();
        assert_eq!(ok_res[0], SteelVal::IntV(42));
        assert_eq!(*trace.lock().unwrap(), vec!["push", "set3d", "pop"]);

        trace.lock().unwrap().clear();
        let err_res = vm.run("(with-camera-3d (camera-3d-default) (lambda () (error \"boom\")))");
        assert!(err_res.is_err());
        assert_eq!(*trace.lock().unwrap(), vec!["push", "set3d", "pop"]);
    }
}

//! Headless integration tests for `scheme_3d`.
//!
//! Because Macroquad's rendering, input, and window functions require an active OpenGL context
//! created by `macroquad::Window`, these tests wrap [`scheme_3d::build_macroquad_module`] and
//! override only the GL-context-dependent calls with headless no-ops/mock values while keeping
//! all custom types (`MqVec2`, `MqVec3`, `MqColor`, `MqCamera2D`, `MqCamera3D`), math, color,
//! camera, `PersistStore`, `scheme3d/prelude`, and multi-file module resolution 100% real.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use macroquad::math::Vec2;
use scheme_3d::runtime::{PersistStore, ReloadStatus, RuntimeConfig, ScriptHost};
use scheme_3d::types::{
    steel_to_color, steel_to_f32, steel_to_symbol_or_str, steel_to_vec2, steel_to_vec3,
    vec2_into_steelval, MqCamera2D, MqCamera3D,
};
use scheme_3d::{build_macroquad_module, PRELUDE_SCM};
use steel::rvals::{AsRefSteelVal, Result as SteelResult, SteelVal};
use steel::steel_vm::builtin::BuiltInModule;
use steel::steel_vm::register_fn::RegisterFn;

static CLEAR_BG_COUNT: AtomicUsize = AtomicUsize::new(0);
static SHAPES_2D_COUNT: AtomicUsize = AtomicUsize::new(0);
static MODELS_3D_COUNT: AtomicUsize = AtomicUsize::new(0);
static CAMERA_OPS_COUNT: AtomicUsize = AtomicUsize::new(0);
static TEXT_OPS_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Builds the full `macroquad` module with real types/math/color/camera/persist bindings,
/// and replaces only the functions that touch Macroquad's thread-local OpenGL/window context
/// so that example scripts can execute `(init)`, `(update dt)`, and `(draw)` headlessly.
fn build_headless_test_module(persist: &PersistStore) -> BuiltInModule {
    let mut m = build_macroquad_module(persist);

    // --- 2D shapes (validate argument types & count draw calls) ---
    m.register_fn("clear-background", |color: SteelVal| -> SteelResult<()> {
        let _ = steel_to_color(&color)?;
        CLEAR_BG_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    m.register_fn(
        "draw-line",
        |x1: SteelVal,
         y1: SteelVal,
         x2: SteelVal,
         y2: SteelVal,
         thickness: SteelVal,
         color: SteelVal|
         -> SteelResult<()> {
            let _ = (
                steel_to_f32(&x1)?,
                steel_to_f32(&y1)?,
                steel_to_f32(&x2)?,
                steel_to_f32(&y2)?,
                steel_to_f32(&thickness)?,
                steel_to_color(&color)?,
            );
            SHAPES_2D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-rectangle",
        |x: SteelVal, y: SteelVal, w: SteelVal, h: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_f32(&x)?,
                steel_to_f32(&y)?,
                steel_to_f32(&w)?,
                steel_to_f32(&h)?,
                steel_to_color(&color)?,
            );
            SHAPES_2D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-rectangle-lines",
        |x: SteelVal,
         y: SteelVal,
         w: SteelVal,
         h: SteelVal,
         thickness: SteelVal,
         color: SteelVal|
         -> SteelResult<()> {
            let _ = (
                steel_to_f32(&x)?,
                steel_to_f32(&y)?,
                steel_to_f32(&w)?,
                steel_to_f32(&h)?,
                steel_to_f32(&thickness)?,
                steel_to_color(&color)?,
            );
            SHAPES_2D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-circle",
        |x: SteelVal, y: SteelVal, r: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_f32(&x)?,
                steel_to_f32(&y)?,
                steel_to_f32(&r)?,
                steel_to_color(&color)?,
            );
            SHAPES_2D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-circle-lines",
        |x: SteelVal,
         y: SteelVal,
         r: SteelVal,
         thickness: SteelVal,
         color: SteelVal|
         -> SteelResult<()> {
            let _ = (
                steel_to_f32(&x)?,
                steel_to_f32(&y)?,
                steel_to_f32(&r)?,
                steel_to_f32(&thickness)?,
                steel_to_color(&color)?,
            );
            SHAPES_2D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-poly",
        |x: SteelVal,
         y: SteelVal,
         sides: SteelVal,
         radius: SteelVal,
         rotation: SteelVal,
         color: SteelVal|
         -> SteelResult<()> {
            let _ = (
                steel_to_f32(&x)?,
                steel_to_f32(&y)?,
                steel_to_f32(&sides)?,
                steel_to_f32(&radius)?,
                steel_to_f32(&rotation)?,
                steel_to_color(&color)?,
            );
            SHAPES_2D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-poly-lines",
        |x: SteelVal,
         y: SteelVal,
         sides: SteelVal,
         radius: SteelVal,
         rotation: SteelVal,
         thickness: SteelVal,
         color: SteelVal|
         -> SteelResult<()> {
            let _ = (
                steel_to_f32(&x)?,
                steel_to_f32(&y)?,
                steel_to_f32(&sides)?,
                steel_to_f32(&radius)?,
                steel_to_f32(&rotation)?,
                steel_to_f32(&thickness)?,
                steel_to_color(&color)?,
            );
            SHAPES_2D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );

    // --- Camera state stack operations ---
    m.register_fn("set-camera-3d", |cam: SteelVal| -> SteelResult<()> {
        let _ = MqCamera3D::as_ref(&cam)?;
        CAMERA_OPS_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    m.register_fn("set-camera-2d", |cam: SteelVal| -> SteelResult<()> {
        let _ = MqCamera2D::as_ref(&cam)?;
        CAMERA_OPS_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    m.register_fn("set-default-camera", || -> SteelResult<()> {
        CAMERA_OPS_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    m.register_fn("push-camera-state", || -> SteelResult<()> {
        CAMERA_OPS_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    m.register_fn("pop-camera-state", || -> SteelResult<()> {
        CAMERA_OPS_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });

    // --- 3D models ---
    m.register_fn(
        "draw-cube",
        |pos: SteelVal, size: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_vec3(&pos)?,
                steel_to_vec3(&size)?,
                steel_to_color(&color)?,
            );
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-cube-wires",
        |pos: SteelVal, size: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_vec3(&pos)?,
                steel_to_vec3(&size)?,
                steel_to_color(&color)?,
            );
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-sphere",
        |center: SteelVal, radius: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_vec3(&center)?,
                steel_to_f32(&radius)?,
                steel_to_color(&color)?,
            );
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "draw-sphere-wires",
        |center: SteelVal, radius: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_vec3(&center)?,
                steel_to_f32(&radius)?,
                steel_to_color(&color)?,
            );
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_value(
        "draw-cylinder",
        SteelVal::FuncV(|args: &[SteelVal]| -> SteelResult<SteelVal> {
            let color_idx = if args.len() == 6 { 5 } else { 4 };
            let _ = (
                steel_to_vec3(&args[0])?,
                steel_to_f32(&args[1])?,
                steel_to_f32(&args[2])?,
                steel_to_f32(&args[3])?,
                steel_to_color(&args[color_idx])?,
            );
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(SteelVal::Void)
        }),
    );
    m.register_value(
        "draw-cylinder-wires",
        SteelVal::FuncV(|args: &[SteelVal]| -> SteelResult<SteelVal> {
            let color_idx = if args.len() == 6 { 5 } else { 4 };
            let _ = (
                steel_to_vec3(&args[0])?,
                steel_to_f32(&args[1])?,
                steel_to_f32(&args[2])?,
                steel_to_f32(&args[3])?,
                steel_to_color(&args[color_idx])?,
            );
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(SteelVal::Void)
        }),
    );
    m.register_fn(
        "draw-plane",
        |center: SteelVal, size: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_vec3(&center)?,
                steel_to_vec2(&size)?,
                steel_to_color(&color)?,
            );
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_value(
        "draw-grid",
        SteelVal::FuncV(|_args: &[SteelVal]| -> SteelResult<SteelVal> {
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(SteelVal::Void)
        }),
    );
    m.register_fn(
        "draw-line-3d",
        |start: SteelVal, end: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_vec3(&start)?,
                steel_to_vec3(&end)?,
                steel_to_color(&color)?,
            );
            MODELS_3D_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );

    // --- Input (headless mock values) ---
    m.register_fn("is-key-down?", |key: SteelVal| -> SteelResult<bool> {
        let _ = steel_to_symbol_or_str(&key)?;
        Ok(false)
    });
    m.register_fn("is-key-pressed?", |key: SteelVal| -> SteelResult<bool> {
        let _ = steel_to_symbol_or_str(&key)?;
        Ok(false)
    });
    m.register_fn("is-key-released?", |key: SteelVal| -> SteelResult<bool> {
        let _ = steel_to_symbol_or_str(&key)?;
        Ok(false)
    });
    m.register_fn("is-mouse-button-down?", |btn: SteelVal| -> SteelResult<bool> {
        let _ = steel_to_symbol_or_str(&btn)?;
        Ok(false)
    });
    m.register_fn(
        "is-mouse-button-pressed?",
        |btn: SteelVal| -> SteelResult<bool> {
            let _ = steel_to_symbol_or_str(&btn)?;
            Ok(false)
        },
    );
    m.register_fn("mouse-position", || -> SteelResult<SteelVal> {
        vec2_into_steelval(Vec2::new(640.0, 360.0))
    });
    m.register_fn("mouse-delta-position", || -> SteelResult<SteelVal> {
        vec2_into_steelval(Vec2::ZERO)
    });
    m.register_fn("mouse-wheel", || -> SteelResult<SteelVal> {
        vec2_into_steelval(Vec2::ZERO)
    });

    // --- Text & Window ---
    m.register_fn(
        "draw-text",
        |_text: String, x: SteelVal, y: SteelVal, size: SteelVal, color: SteelVal| -> SteelResult<()> {
            let _ = (
                steel_to_f32(&x)?,
                steel_to_f32(&y)?,
                steel_to_f32(&size)?,
                steel_to_color(&color)?,
            );
            TEXT_OPS_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        },
    );
    m.register_fn(
        "measure-text",
        |text: String, font_size: SteelVal| -> SteelResult<SteelVal> {
            let sz = steel_to_f32(&font_size)?;
            vec2_into_steelval(Vec2::new(text.len() as f32 * sz * 0.5, sz))
        },
    );
    m.register_value(
        "draw-fps",
        SteelVal::FuncV(|_args: &[SteelVal]| -> SteelResult<SteelVal> {
            TEXT_OPS_COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(SteelVal::Void)
        }),
    );
    m.register_fn("screen-width", || -> f64 { 1280.0 });
    m.register_fn("screen-height", || -> f64 { 720.0 });
    m.register_fn("get-fps", || -> i64 { 60 });
    m.register_fn("get-frame-time", || -> f64 { 0.016 });
    m.register_fn("get-time", || -> f64 { 1.25 });

    m
}

#[test]
fn test_example_hello_2d_end_to_end() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script_dir = manifest_dir.join("examples/hello_2d");

    let before_clear = CLEAR_BG_COUNT.load(Ordering::Relaxed);
    let before_shapes = SHAPES_2D_COUNT.load(Ordering::Relaxed);
    let before_text = TEXT_OPS_COUNT.load(Ordering::Relaxed);

    let config = RuntimeConfig::new(script_dir).with_watch(false);
    let mut host = ScriptHost::load(config, PRELUDE_SCM, build_headless_test_module)
        .expect("failed to load examples/hello_2d");

    host.update(0.016).expect("hello_2d update failed");
    host.draw().expect("hello_2d draw failed");

    assert!(host.last_error().is_none());
    assert!(CLEAR_BG_COUNT.load(Ordering::Relaxed) > before_clear);
    assert!(SHAPES_2D_COUNT.load(Ordering::Relaxed) >= before_shapes + 5);
    assert!(TEXT_OPS_COUNT.load(Ordering::Relaxed) >= before_text + 3);
}

#[test]
fn test_example_basic_3d_multi_module_and_camera_scope() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script_dir = manifest_dir.join("examples/basic_3d");

    let before_clear = CLEAR_BG_COUNT.load(Ordering::Relaxed);
    let before_cam = CAMERA_OPS_COUNT.load(Ordering::Relaxed);
    let before_models = MODELS_3D_COUNT.load(Ordering::Relaxed);
    let before_text = TEXT_OPS_COUNT.load(Ordering::Relaxed);

    let config = RuntimeConfig::new(script_dir).with_watch(false);
    let mut host = ScriptHost::load(config, PRELUDE_SCM, build_headless_test_module)
        .expect("failed to load examples/basic_3d");

    // Verify init populated PersistStore
    assert!(host.persist_store().contains("cam-yaw"));
    assert!(host.persist_store().contains("cam-pitch"));
    assert!(host.persist_store().contains("cam-dist"));

    // Simulate two frames of update + draw
    host.update(0.05).expect("basic_3d frame 1 update failed");
    host.draw().expect("basic_3d frame 1 draw failed");
    host.update(0.05).expect("basic_3d frame 2 update failed");
    host.draw().expect("basic_3d frame 2 draw failed");

    assert!(host.last_error().is_none());
    assert!(CLEAR_BG_COUNT.load(Ordering::Relaxed) >= before_clear + 2);
    // with-camera-3d calls push-camera-state, set-camera-3d, pop-camera-state each frame
    assert!(CAMERA_OPS_COUNT.load(Ordering::Relaxed) >= before_cam + 6);
    assert!(MODELS_3D_COUNT.load(Ordering::Relaxed) >= before_models + 16);
    assert!(TEXT_OPS_COUNT.load(Ordering::Relaxed) >= before_text + 10);

    // Force a hot reload and verify state is preserved across Engine rebuilds
    match host.try_reload() {
        ReloadStatus::Reloaded => {}
        other => panic!("expected Reloaded on examples/basic_3d, got {other:?}"),
    }
    host.update(0.05).expect("basic_3d post-reload update failed");
    host.draw().expect("basic_3d post-reload draw failed");
    assert!(host.last_error().is_none());
}

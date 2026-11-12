use steel::steel_vm::engine::Engine;
use steel::{SteelErr, SteelVal};

use crate::runtime::loader::{load_project_sources, MapSourceModuleResolver};
use crate::runtime::{
    FrameErrorState, ModuleBuilder, PersistStore, ReloadStatus, RuntimeConfig, ScriptWatcher,
};

pub struct ScriptHost {
    config: RuntimeConfig,
    persist: PersistStore,
    prelude_scm: String,
    module_builder: ModuleBuilder,
    engine: Engine,
    error_state: FrameErrorState,
    last_reload_error: Option<String>,
    reload_count: u64,
}

fn format_engine_error(engine: &Engine, err: SteelErr) -> String {
    let fallback = err.to_string();
    engine.raise_error_to_string(err).unwrap_or(fallback)
}

impl ScriptHost {
    pub fn load(
        config: RuntimeConfig,
        prelude_scm: impl Into<String>,
        module_builder: ModuleBuilder,
    ) -> Result<Self, String> {
        let prelude_scm = prelude_scm.into();
        let persist = PersistStore::new();
        let engine = Self::build_candidate_engine(&config, &persist, &prelude_scm, module_builder, false)?;
        Ok(Self {
            config,
            persist,
            prelude_scm,
            module_builder,
            engine,
            error_state: FrameErrorState::default(),
            last_reload_error: None,
            reload_count: 0,
        })
    }

    fn build_candidate_engine(
        config: &RuntimeConfig,
        persist: &PersistStore,
        prelude_scm: &str,
        module_builder: ModuleBuilder,
        is_reload: bool,
    ) -> Result<Engine, String> {
        let sources = load_project_sources(&config.script_dir, &config.entry_file)?;
        let mut engine = Engine::new();

        #[cfg(not(target_arch = "wasm32"))]
        engine.add_search_directory(sources.root_dir.clone());

        if !sources.virtual_modules.is_empty() {
            engine.register_source_module_resolver(MapSourceModuleResolver::new(
                sources.virtual_modules.clone(),
            ));
        }

        let builtin = (module_builder)(persist);
        engine.register_module(builtin);

        let has_prelude = !prelude_scm.trim().is_empty();
        if has_prelude {
            engine.register_steel_module("scheme3d/prelude".to_string(), prelude_scm.to_owned());
        }

        let bootstrap = if has_prelude {
            "(require-builtin macroquad)\n(require \"scheme3d/prelude\")"
        } else {
            "(require-builtin macroquad)"
        };

        engine
            .compile_and_run_raw_program(bootstrap)
            .map_err(|e| format_engine_error(&engine, e))?;

        engine
            .compile_and_run_raw_program_with_path(sources.entry_source, sources.entry_path)
            .map_err(|e| format_engine_error(&engine, e))?;

        if is_reload {
            if engine.global_exists("on-reload") {
                engine
                    .call_function_by_name_with_args("on-reload", vec![])
                    .map_err(|e| format_engine_error(&engine, e))?;
            }
        } else if engine.global_exists("init") {
            engine
                .call_function_by_name_with_args("init", vec![])
                .map_err(|e| format_engine_error(&engine, e))?;
        }

        Ok(engine)
    }

    pub fn try_reload(&mut self) -> ReloadStatus {
        match Self::build_candidate_engine(
            &self.config,
            &self.persist,
            &self.prelude_scm,
            self.module_builder,
            true,
        ) {
            Ok(new_engine) => {
                self.engine = new_engine;
                self.error_state.clear();
                self.last_reload_error = None;
                self.reload_count += 1;
                eprintln!(
                    "[scheme_3d] Hot-reloaded {:?} (#{})",
                    self.config.entry_path(),
                    self.reload_count
                );
                ReloadStatus::Reloaded
            }
            Err(err_msg) => {
                eprintln!(
                    "[scheme_3d] Hot-reload failed (keeping previous engine running):\n{}",
                    err_msg
                );
                self.last_reload_error = Some(err_msg.clone());
                ReloadStatus::Failed(err_msg)
            }
        }
    }

    pub fn poll_and_reload(&mut self, watcher: &mut ScriptWatcher) -> ReloadStatus {
        if watcher.poll_changed() {
            self.try_reload()
        } else {
            ReloadStatus::None
        }
    }

    pub fn update(&mut self, dt: f32) -> Result<(), String> {
        if !self.engine.global_exists("update") {
            self.error_state.clear_update();
            return Ok(());
        }

        match self
            .engine
            .call_function_by_name_with_args("update", vec![SteelVal::NumV(dt as f64)])
        {
            Ok(_) => {
                self.error_state.clear_update();
                Ok(())
            }
            Err(e) => {
                let msg = format_engine_error(&self.engine, e);
                if self.error_state.report_update_if_new(&msg) {
                    eprintln!("[scheme_3d] Runtime error in (update dt):\n{}", msg);
                }
                Err(msg)
            }
        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        if !self.engine.global_exists("draw") {
            self.error_state.clear_draw();
            return Ok(());
        }

        match self
            .engine
            .call_function_by_name_with_args("draw", vec![])
        {
            Ok(_) => {
                self.error_state.clear_draw();
                Ok(())
            }
            Err(e) => {
                let msg = format_engine_error(&self.engine, e);
                if self.error_state.report_draw_if_new(&msg) {
                    eprintln!("[scheme_3d] Runtime error in (draw):\n{}", msg);
                }
                Err(msg)
            }
        }
    }

    pub fn persist_store(&self) -> &PersistStore {
        &self.persist
    }

    pub fn has_global(&self, name: &str) -> bool {
        self.engine.global_exists(name)
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_reload_error
            .as_deref()
            .or_else(|| self.error_state.last_error())
    }

    pub fn reload_count(&self) -> u64 {
        self.reload_count
    }

    pub fn eval_snippet(&mut self, code: &str) -> Result<Vec<SteelVal>, String> {
        self.engine
            .run(code.to_string())
            .map_err(|e| format_engine_error(&self.engine, e))
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::build_macroquad_module;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempProject(PathBuf);
    impl TempProject {
        fn new() -> Self {
            let id = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "scheme_3d_engine_test_{}_{}",
                std::process::id(),
                id
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn write(&self, rel: &str, content: &str) {
            let full = self.0.join(rel);
            if let Some(parent) = full.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::write(full, content).unwrap();
        }
    }
    impl Drop for TempProject {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn test_require_provide_hot_reload_and_error_recovery() {
        let proj = TempProject::new();
        proj.write(
            "helper.scm",
            r#"
            (require-builtin macroquad)
            (provide step-pos)
            (define (step-pos p dt)
              (vec3+ p (vec3 (* 10 dt) 0 0)))
            "#,
        );
        proj.write(
            "main.scm",
            r#"
            (require "helper.scm")
            (define (init)
              (state-set! 'inits (+ 1 (state-get 'inits 0)))
              (state-set! 'pos (vec3 0 0 0)))
            (define (on-reload)
              (state-set! 'reloads (+ 1 (state-get 'reloads 0))))
            (define (update dt)
              (state-set! 'pos (step-pos (state-get 'pos (vec3 0 0 0)) dt)))
            (define (draw)
              (state-set! 'draws (+ 1 (state-get 'draws 0))))
            "#,
        );

        let cfg = RuntimeConfig::new(proj.0.clone()).with_watch(false);
        let mut host = ScriptHost::load(
            cfg,
            include_str!("../../scheme/prelude.scm"),
            build_macroquad_module,
        )
        .unwrap();

        assert!(host.has_global("update"));
        assert!(host.has_global("draw"));
        host.update(0.5).unwrap();
        host.draw().unwrap();
        assert_eq!(
            host.eval_snippet("(vec3-x (state-get 'pos (vec3 0 0 0)))")
                .unwrap()[0],
            SteelVal::NumV(5.0)
        );
        assert_eq!(
            host.eval_snippet("(state-get 'inits 0)").unwrap()[0],
            SteelVal::IntV(1)
        );

        // Hot reload with updated helper multiplier
        proj.write(
            "helper.scm",
            r#"
            (require-builtin macroquad)
            (provide step-pos)
            (define (step-pos p dt)
              (vec3+ p (vec3 (* 100 dt) 0 0)))
            "#,
        );
        assert_eq!(host.try_reload(), ReloadStatus::Reloaded);
        assert_eq!(host.reload_count(), 1);
        assert!(host.last_error().is_none());

        host.update(0.5).unwrap();
        assert_eq!(
            host.eval_snippet("(vec3-x (state-get 'pos (vec3 0 0 0)))")
                .unwrap()[0],
            SteelVal::NumV(55.0)
        );
        // init did not re-run; on-reload ran once
        assert_eq!(
            host.eval_snippet("(state-get 'inits 0)").unwrap()[0],
            SteelVal::IntV(1)
        );
        assert_eq!(
            host.eval_snippet("(state-get 'reloads 0)").unwrap()[0],
            SteelVal::IntV(1)
        );

        // Introduce syntax error in main.scm -> reload fails, old engine continues working!
        proj.write("main.scm", "(define (update dt) (+ 1 ");
        match host.try_reload() {
            ReloadStatus::Failed(_) => {}
            other => panic!("expected Failed, got {other:?}"),
        }
        assert!(host.last_error().is_some());
        assert_eq!(host.reload_count(), 1);

        // Old engine still updates cleanly!
        host.update(0.5).unwrap();
        assert_eq!(
            host.eval_snippet("(vec3-x (state-get 'pos (vec3 0 0 0)))")
                .unwrap()[0],
            SteelVal::NumV(105.0)
        );
    }
}

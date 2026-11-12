pub mod engine;
pub mod loader;
pub mod persist;
pub mod watcher;

use std::path::PathBuf;
use steel::steel_vm::builtin::BuiltInModule;

pub use engine::ScriptHost;
pub use persist::{PersistStore, PersistValue};
pub use watcher::ScriptWatcher;

pub type ModuleBuilder = fn(&PersistStore) -> BuiltInModule;

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    pub script_dir: PathBuf,
    pub entry_file: String,
    pub watch_enabled: bool,
}

impl RuntimeConfig {
    pub fn new(script_dir: PathBuf) -> Self {
        Self {
            script_dir,
            entry_file: "main.scm".to_string(),
            watch_enabled: true,
        }
    }

    pub fn with_watch(mut self, watch_enabled: bool) -> Self {
        self.watch_enabled = watch_enabled;
        self
    }

    pub fn entry_path(&self) -> PathBuf {
        self.script_dir.join(&self.entry_file)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReloadStatus {
    None,
    Reloaded,
    Failed(String),
}

#[derive(Clone, Debug, Default)]
pub struct FrameErrorState {
    pub last_update_error: Option<String>,
    pub last_draw_error: Option<String>,
}

impl FrameErrorState {
    pub fn clear(&mut self) {
        self.last_update_error = None;
        self.last_draw_error = None;
    }

    pub fn report_update_if_new(&mut self, err: &str) -> bool {
        if self.last_update_error.as_deref() == Some(err) {
            false
        } else {
            self.last_update_error = Some(err.to_owned());
            true
        }
    }

    pub fn clear_update(&mut self) {
        self.last_update_error = None;
    }

    pub fn report_draw_if_new(&mut self, err: &str) -> bool {
        if self.last_draw_error.as_deref() == Some(err) {
            false
        } else {
            self.last_draw_error = Some(err.to_owned());
            true
        }
    }

    pub fn clear_draw(&mut self) {
        self.last_draw_error = None;
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_update_error
            .as_deref()
            .or(self.last_draw_error.as_deref())
    }
}

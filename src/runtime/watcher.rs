use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{channel, Receiver};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

#[cfg(not(target_arch = "wasm32"))]
const DEBOUNCE_DURATION: Duration = Duration::from_millis(80);

#[cfg(not(target_arch = "wasm32"))]
fn is_relevant_scm_event(event: &Event) -> bool {
    if matches!(event.kind, EventKind::Access(_)) {
        return false;
    }
    event.paths.iter().any(|path| {
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            return false;
        };
        if name.starts_with('.') || name.ends_with('~') || name.ends_with(".swp") || name.ends_with(".tmp") {
            return false;
        }
        path.extension().and_then(|e| e.to_str()) == Some("scm")
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub struct ScriptWatcher {
    _watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<Event>>,
    last_event_at: Option<Instant>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ScriptWatcher {
    pub fn new(root: &Path) -> Result<Self, String> {
        let (tx, rx) = channel();
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default(),
        )
        .map_err(|e| format!("Failed to initialize file watcher for {:?}: {e}", root))?;

        watcher
            .watch(root, RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch directory {:?}: {e}", root))?;

        Ok(Self {
            _watcher: watcher,
            rx,
            last_event_at: None,
        })
    }

    pub fn poll_changed(&mut self) -> bool {
        while let Ok(res) = self.rx.try_recv() {
            if let Ok(event) = res {
                if is_relevant_scm_event(&event) {
                    self.last_event_at = Some(Instant::now());
                }
            }
        }

        if let Some(last) = self.last_event_at {
            if last.elapsed() >= DEBOUNCE_DURATION {
                self.last_event_at = None;
                return true;
            }
        }
        false
    }
}

#[cfg(target_arch = "wasm32")]
pub struct ScriptWatcher;

#[cfg(target_arch = "wasm32")]
impl ScriptWatcher {
    pub fn new(_root: &Path) -> Result<Self, String> {
        Ok(Self)
    }

    pub fn poll_changed(&mut self) -> bool {
        false
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempDir(PathBuf);
    impl TempDir {
        fn new() -> Self {
            let id = COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!(
                "scheme_3d_watcher_test_{}_{}",
                std::process::id(),
                id
            ));
            std::fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn test_scm_event_filter_and_debounce() {
        let tmp = TempDir::new();
        let scm_file = tmp.0.join("main.scm");
        let txt_file = tmp.0.join("notes.txt");
        std::fs::write(&scm_file, "(define x 1)").unwrap();

        let mut watcher = ScriptWatcher::new(&tmp.0).unwrap();
        std::fs::write(&txt_file, "ignored").unwrap();
        std::thread::sleep(Duration::from_millis(110));
        assert!(!watcher.poll_changed());

        std::fs::write(&scm_file, "(define x 2)").unwrap();
        // Wait for OS event + debounce window
        let start = Instant::now();
        let mut triggered = false;
        while start.elapsed() < Duration::from_secs(2) {
            if watcher.poll_changed() {
                triggered = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(triggered, "watcher should detect .scm change after debounce");
        assert!(!watcher.poll_changed(), "watcher should reset after firing");
    }
}

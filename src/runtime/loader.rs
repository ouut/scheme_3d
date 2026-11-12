use std::collections::HashMap;
use std::path::{Path, PathBuf};

use steel::compiler::modules::SourceModuleResolver;

#[derive(Clone, Debug)]
pub struct ProjectSources {
    pub root_dir: PathBuf,
    pub entry_path: PathBuf,
    pub entry_source: String,
    pub virtual_modules: HashMap<String, String>,
}

#[derive(Clone, Debug, Default)]
pub struct MapSourceModuleResolver {
    modules: HashMap<String, String>,
}

impl MapSourceModuleResolver {
    pub fn new(modules: HashMap<String, String>) -> Self {
        Self { modules }
    }

    fn normalize_key(key: &str) -> &str {
        key.strip_prefix("./").unwrap_or(key)
    }
}

impl SourceModuleResolver for MapSourceModuleResolver {
    fn resolve(&self, key: &str) -> Option<String> {
        let norm = Self::normalize_key(key);
        self.modules
            .get(key)
            .or_else(|| self.modules.get(norm))
            .cloned()
    }

    fn exists(&self, key: &str) -> bool {
        let norm = Self::normalize_key(key);
        self.modules.contains_key(key) || self.modules.contains_key(norm)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_project_sources(script_dir: &Path, entry_file: &str) -> Result<ProjectSources, String> {
    if !script_dir.exists() {
        return Err(format!("Script directory does not exist: {:?}", script_dir));
    }
    if !script_dir.is_dir() {
        return Err(format!("Script path is not a directory: {:?}", script_dir));
    }
    let canonical_root = std::fs::canonicalize(script_dir)
        .map_err(|e| format!("Failed to resolve script directory {:?}: {e}", script_dir))?;
    let entry_path = canonical_root.join(entry_file);
    if !entry_path.is_file() {
        return Err(format!("Entry script not found: {:?}", entry_path));
    }
    let entry_source = std::fs::read_to_string(&entry_path)
        .map_err(|e| format!("Failed to read entry script {:?}: {e}", entry_path))?;
    Ok(ProjectSources {
        root_dir: canonical_root,
        entry_path,
        entry_source,
        virtual_modules: HashMap::new(),
    })
}

#[cfg(target_arch = "wasm32")]
pub fn load_project_sources(script_dir: &Path, entry_file: &str) -> Result<ProjectSources, String> {
    Err(format!(
        "WASM virtual script loading for {:?}/{} is planned for Phase 3",
        script_dir, entry_file
    ))
}

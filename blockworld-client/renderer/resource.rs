use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use blockworld_utils::Identifier;
use once_cell::sync::Lazy;

/// A single source of game assets (filesystem dir, zip, embedded bytes, mod).
pub trait PackSource: Send + Sync {
    /// Return raw bytes for a resource, or None if not provided by this source.
    fn get(&self, id: &Identifier) -> Option<Vec<u8>>;

    /// List all identifiers under `namespace:prefix/`.
    fn list(&self, namespace: &str, prefix: &str) -> Vec<Identifier>;
}

/// Merged view of multiple PackSources.
/// Last-added source has highest priority.
pub struct ResourceManager {
    sources: Vec<Box<dyn PackSource>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    pub fn add_source(&mut self, source: impl PackSource + 'static) {
        self.sources.push(Box::new(source));
    }

    /// Walk sources bottom-to-top, return first match.
    pub fn get(&self, id: &Identifier) -> Option<Vec<u8>> {
        self.sources.iter().rev().find_map(|s| s.get(id))
    }

    /// All identifiers across sources. Higher-priority sources win for duplicates.
    pub fn list(&self, namespace: &str, prefix: &str) -> Vec<Identifier> {
        let mut seen = HashMap::new();
        for source in &self.sources {
            for id in source.list(namespace, prefix) {
                seen.entry(id.clone()).or_insert(id);
            }
        }
        seen.into_values().collect()
    }
}

// ── FilesystemSource ────────────────────────────────────────────

pub struct FilesystemSource {
    root: PathBuf,
}

impl FilesystemSource {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    fn resolve(&self, id: &Identifier) -> PathBuf {
        self.root
            .join("assets")
            .join(id.get_namespace())
            .join(id.get_path())
    }
}

impl PackSource for FilesystemSource {
    fn get(&self, id: &Identifier) -> Option<Vec<u8>> {
        std::fs::read(self.resolve(id)).ok()
    }

    fn list(&self, namespace: &str, prefix: &str) -> Vec<Identifier> {
        let dir = self
            .root
            .join("assets")
            .join(namespace)
            .join(prefix);
        let mut out = vec![];
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let id = Identifier::new(
                            &format!("{}:{}/{}", namespace, prefix, stem),
                        );
                        out.push(id);
                    }
                }
            }
        }
        out
    }
}

// ── EmbeddedSource ──────────────────────────────────────────────

pub struct EmbeddedSource {
    data: HashMap<Identifier, &'static [u8]>,
}

impl EmbeddedSource {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn with(mut self, id: Identifier, bytes: &'static [u8]) -> Self {
        self.data.insert(id, bytes);
        self
    }
}

impl PackSource for EmbeddedSource {
    fn get(&self, id: &Identifier) -> Option<Vec<u8>> {
        self.data.get(id).map(|b| b.to_vec())
    }

    fn list(&self, _namespace: &str, _prefix: &str) -> Vec<Identifier> {
        self.data.keys().cloned().collect()
    }
}

// ── Global singleton ────────────────────────────────────────────

pub static RESOURCE_MANAGER: Lazy<Mutex<ResourceManager>> = Lazy::new(|| {
    let mut rm = ResourceManager::new();

    // Embedded shaders (lowest priority)
    rm.add_source(
        EmbeddedSource::new()
            .with(
                Identifier::new("minecraft:assets/shaders/default_shader.wgsl"),
                include_bytes!("shaders/default_shader.wgsl"),
            )
            .with(
                Identifier::new("minecraft:assets/shaders/wireframe_shader.wgsl"),
                include_bytes!("shaders/wireframe_shader.wgsl"),
            ),
    );

    // Filesystem (highest priority — can override embedded)
    rm.add_source(FilesystemSource::new("."));

    Mutex::new(rm)
});

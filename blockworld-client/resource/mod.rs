//! Game resource management system.
//!
//! ## Architecture
//!
//! Minecraft uses a layered resource system:
//!   PackResources (single source)
//!     → ResourceManager (merged view of all sources)
//!       → Consumers (TextureManager, SoundManager, etc.)
//!
//! Our equivalents:
//!   PackSource trait   ← PackResources
//!   ResourceManager    ← MultiPackResourceManager
//!   RESOURCE_MANAGER   ← Minecraft.getInstance().getResourceManager()
//!
//! ## Priority
//!
//! Last-added source has highest priority. Currently:
//!   1. FilesystemSource(".")  — highest, can override embedded
//!   2. EmbeddedSource         — lowest, shaders baked into binary
//!
//! Mods register via:
//!   RESOURCE_MANAGER.lock().unwrap().add_source(FilesystemSource::new("mod_foo/"));

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use blockworld_utils::Identifier;
use once_cell::sync::Lazy;

// -- PackSource trait -------------------------------------------------------

/// A single source of game assets.
///
/// Equivalent to Minecraft's `PackResources` interface.
/// Implementations: `FilesystemSource` (disk), `EmbeddedSource` (binary),
/// future: `ZipSource` (resource packs), mod-provided sources.
pub trait PackSource: Send + Sync {
    /// Return raw bytes for a resource, or `None` if not provided by this source.
    fn get(&self, id: &Identifier) -> Option<Vec<u8>>;

    /// List all identifiers under `namespace:prefix/`.
    ///
    /// Example: `list("blockworld", "textures/block")` returns
    /// `[blockworld:textures/block/stone, blockworld:textures/block/dirt, ...]`.
    fn list(&self, namespace: &str, prefix: &str) -> Vec<Identifier>;
}

// -- ResourceManager --------------------------------------------------------

/// Merged view of multiple `PackSource`s.
///
/// Walk sources in reverse order (last-added = highest priority).
/// When multiple sources provide the same resource identifier,
/// the higher-priority source wins.
pub struct ResourceManager {
    sources: Vec<Box<dyn PackSource>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    /// Append a source. Later additions override earlier ones.
    pub fn add_source(&mut self, source: impl PackSource + 'static) {
        self.sources.push(Box::new(source));
    }

    /// Walk sources from highest to lowest priority, return first match.
    pub fn get(&self, id: &Identifier) -> Option<Vec<u8>> {
        self.sources.iter().rev().find_map(|s| s.get(id))
    }

    /// Collect all identifiers across sources.
    /// Higher-priority sources win for duplicates (last-added overwrites).
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

// -- FilesystemSource -------------------------------------------------------

/// Reads assets from the filesystem under `{root}/assets/{namespace}/{path}`.
///
/// Example: for `Identifier("blockworld:textures/block/stone.png")`,
/// reads `{root}/assets/blockworld/textures/block/stone.png`.
pub struct FilesystemSource {
    root: PathBuf,
}

impl FilesystemSource {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    /// Convert an identifier to a filesystem path:
    /// `{root}/assets/{namespace}/{path}`
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
        let dir = self.root.join("assets").join(namespace).join(prefix);
        let mut out = vec![];
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                        let id = Identifier::new(&format!("{}:{}/{}", namespace, prefix, name));
                        out.push(id);
                    }
                }
            }
        }
        out
    }
}

// -- EmbeddedSource ---------------------------------------------------------

/// Provides resources baked into the binary at compile time via `include_bytes!`.
///
/// Used for shaders so the client binary is self-contained without
/// requiring external shader files at runtime. Filesystem sources
/// override embedded ones at higher priority.
pub struct EmbeddedSource {
    data: HashMap<Identifier, &'static [u8]>,
}

impl EmbeddedSource {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Builder-style: associate an identifier with static bytes.
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

// -- Global singleton -------------------------------------------------------

/// The global resource manager.
///
/// Wrapped in `Lazy<Mutex<...>>` so mods can call `lock()` and
/// `add_source()` at runtime to register their own asset directories.
///
/// Priority order (lowest to highest):
///   1. EmbeddedSource — shaders baked into binary
///   2. FilesystemSource(".") — runtime overrides from the working directory
pub static RESOURCE_MANAGER: Lazy<Mutex<ResourceManager>> = Lazy::new(|| {
    let mut rm = ResourceManager::new();

    // Embedded shaders (lowest priority)
    rm.add_source(
        EmbeddedSource::new()
            .with(
                Identifier::new(&format!(
                    "{}:assets/shaders/default_shader.wgsl",
                    blockworld_utils::GAME_NAME
                )),
                include_bytes!("../renderer/shaders/default_shader.wgsl"),
            )
            .with(
                Identifier::new(&format!(
                    "{}:assets/shaders/wireframe_shader.wgsl",
                    blockworld_utils::GAME_NAME
                )),
                include_bytes!("../renderer/shaders/wireframe_shader.wgsl"),
            ),
    );

    // Filesystem (highest priority — can override embedded)
    rm.add_source(FilesystemSource::new("."));

    Mutex::new(rm)
});

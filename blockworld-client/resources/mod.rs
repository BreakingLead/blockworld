use anyhow::*;
use blockworld_utils::resource_location::str;
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use tokio::io::ReadBuf;
struct PackMetadataSection {
    /// Version of the pack format
    ///
    /// - 1 for versions 1.6.1 – 1.8.9
    /// - 2 for versions 1.9 – 1.10.2
    /// - 3 for versions 1.11 – 1.12.2
    /// - 4 for versions 1.13 – 1.14.4
    /// - 5 for versions 1.15 – 1.16.1
    /// - 6 for versions 1.16.2 – 1.16.5
    /// - 7 for versions 1.17.x
    /// - 8 for versions 1.18.x
    /// - etc.
    pub pack_format: i32,
    // TODO: SHOULD BE TEXT COMPONENT
    /// Description of the pack, displayed in the pack selection screen
    pub description: String,
}

const RESOURCE_ROOT: &Path = Path::new("./assets/assets/");

pub enum ResourceType {
    ClientResources,
    ServerData,
}

impl ResourceType {
    fn get_directory_name(&self) -> &'static str {
        match self {
            ResourceType::ClientResources => "assets",
            ResourceType::ServerData => "data",
        }
    }
}

struct Resource {
    location: str,
    map_metadata: HashMap<String, Value>,
}

impl Resource {
    fn get_metadata(&self, key: &str) -> Option<serde_json::Value> {
        self.map_metadata.get(key).cloned()
    }
}

trait ResourcePack {
    fn get_metadata(&self) -> Result<PackMetadataSection>;
    fn resource_exists(&self, resource_path: &str) -> bool;
    fn get_name(&self) -> String;
    /// `getInputStream` in Minecraft
    fn get_read_stream(&self) -> ReadBuf;
}

struct VanillaResourcePack {
    name: String,
}

impl ResourcePack for VanillaResourcePack {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_metadata(&self) -> Result<PackMetadataSection, std::io::Error> {}

    fn resource_exists(&self, location: &Path) -> bool {
        Path::exists(location)
    }

    fn get_read_stream(&self) -> ReadBuf {
        todo!()
    }
}

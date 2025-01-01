use std::{borrow::Borrow, ops::Deref, path::PathBuf};

/// Same as Minecraft's `ResourceLocation` or `Identifier` in yarn mappings.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceLocation {
    id: String,
    // we can't set 2 fields (namespace and path)
    // otherwise we can't turn this into a &str, and it will be a pain that we even can't turn this into a &'static str
}

pub trait HasResourceLocation {
    fn get_id(&self) -> ResourceLocation;
}

impl Default for ResourceLocation {
    fn default() -> Self {
        Self {
            id: "minecraft:air".to_string(),
        }
    }
}

impl ResourceLocation {
    pub fn new(id: &str) -> Self {
        if let Some((_, _)) = id.split_once(":") {
            Self { id: id.to_string() }
        } else {
            log::error!("Invalid ResourceLocation: {}", id);
            Self::default()
        }
    }

    pub fn get_namespace(&self) -> String {
        self.id
            .split_once(":")
            .unwrap_or(("minecraft", "air"))
            .0
            .to_string()
    }

    pub fn get_path(&self) -> String {
        self.id
            .split_once(":")
            .unwrap_or(("minecraft", "air"))
            .1
            .to_string()
    }

    pub fn to_string(&self) -> String {
        self.id.clone()
    }
}

impl From<&str> for ResourceLocation {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl Deref for ResourceLocation {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

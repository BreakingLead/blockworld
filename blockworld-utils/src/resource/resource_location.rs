use std::{borrow::Borrow, ops::Deref, path::PathBuf};

/// Same as Minecraft's `Identifier` in official mappings.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    id: String,
    // we can't set 2 fields (namespace and path)
    // otherwise we can't turn this into a &str, and it will be a pain that we even can't turn this into a &'static str
}

pub trait HasIdentifier {
    fn get_id(&self) -> Identifier;
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            id: "minecraft:air".to_string(),
        }
    }
}

impl Identifier {
    pub fn new(id: &str) -> Self {
        if let Some((_, _)) = id.split_once(":") {
            Self { id: id.to_string() }
        } else {
            log::error!("Invalid Identifier: {}", id);
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

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

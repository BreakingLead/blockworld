use std::path::PathBuf;

/// Same as Minecraft's `ResourceLocation` or `Identifier` in yarn mappings.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceLocation {
    namespace: String,
    path: PathBuf,
}

impl ResourceLocation {
    pub fn new(id: &str) -> Self {
        if let Some((a, b)) = id.split_once(':') {
            Self {
                namespace: a.to_string(),
                path: b.into(),
            }
        } else {
            Self {
                namespace: "minecraft".to_string(),
                path: id.into(),
            }
        }
    }

    pub fn get_namespace(&self) -> String {
        self.namespace.clone()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn with_namespace(self, namespace: String) -> Self {
        Self {
            namespace,
            path: self.path,
        }
    }

    pub fn with_path(self, path: PathBuf) -> Self {
        Self {
            namespace: self.namespace,
            path,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{:?}", self.namespace, self.path)
    }
}

impl From<&str> for ResourceLocation {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl std::fmt::Display for ResourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:?}", self.namespace, self.path)
    }
}

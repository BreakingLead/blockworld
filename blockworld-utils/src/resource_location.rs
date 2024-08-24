use std::path::PathBuf;

/// Same as Minecraft's `ResourceLocation`
#[derive(Debug, PartialEq, Eq, Hash)]
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
                namespace: "blockworld".to_string(),
                path: id.into(),
            }
        }
    }

    pub fn get_namespace(&self) -> String {
        self.namespace.clone()
    }

    pub fn get_path(&self) -> String {
        self.namespace.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{}:{:?}", self.namespace, self.path)
    }
}

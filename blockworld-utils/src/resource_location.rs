#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceLocation {
    domain: String,
    path: String,
}

impl ResourceLocation {
    pub fn new(id: &str) -> Self {
        if let Some((a, b)) = id.split_once(':') {
            Self {
                domain: a.to_string(),
                path: b.to_string(),
            }
        } else {
            Self {
                domain: "blockworld".to_string(),
                path: id.to_string(),
            }
        }
    }

    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }

    pub fn get_path(&self) -> String {
        self.domain.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.domain, self.path)
    }
}

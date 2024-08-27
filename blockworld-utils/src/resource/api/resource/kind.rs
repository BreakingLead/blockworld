use crate::resource::api::ResourceCategory;

/// The type of a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// Resources (`.json`) in `assets/<namespace>/blockstates/`.
    BlockStates,

    /// Resources (`.json`) in `assets/<namespace>/models/block/`.
    BlockModel,

    /// Resources (`.json`) in `assets/<namespace>/models/item/`.
    ItemModel,

    /// Resources (`.png`) in `assets/<namespace>/textures/`.
    Texture,

    /// Resources (`.png.mcmeta`) in `assets/<namespace>/textures/`.
    TextureMeta,
}

impl ResourceKind {
    /// Returns the category of this resource type (assets or data).
    pub fn category(&self) -> ResourceCategory {
        match self {
            Self::BlockStates
            | Self::BlockModel
            | Self::ItemModel
            | Self::Texture
            | Self::TextureMeta => ResourceCategory::Assets,
        }
    }

    /// Returns the file extension used for this resource's file.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let kind = ResourceKind::BlockStates;
    /// assert_eq!(kind.extension(), "json");
    ///
    /// let kind = ResourceKind::Texture;
    /// assert_eq!(kind.extension(), "png");
    ///
    /// let kind = ResourceKind::TextureMeta;
    /// assert_eq!(kind.extension(), "png.mcmeta");
    /// ```
    pub fn extension(&self) -> &'static str {
        match self {
            Self::BlockStates | Self::BlockModel | Self::ItemModel => "json",
            Self::Texture => "png",
            Self::TextureMeta => "png.mcmeta",
        }
    }

    /// Returns the path relative to `assets/<namespace>/` or
    /// `data/<namespace>/` in which resources of this type reside.
    pub fn directory(&self) -> &'static str {
        match self {
            Self::BlockStates => "blockstates",
            Self::BlockModel => "models/block",
            Self::ItemModel => "models/item",
            Self::Texture | Self::TextureMeta => "textures",
        }
    }
}

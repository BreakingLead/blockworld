/// The category of a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceCategory {
    /// Resources located in the `assets/` directory.
    Assets,

    /// Resource located in the `data/` directory.
    Data,
}

impl ResourceCategory {
    /// Returns the name of the top-level directory containing this category of
    /// resource.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let category = ResourceCategory::Assets;
    /// assert_eq!(category.directory(), "assets");
    ///
    /// let category = ResourceCategory::Data;
    /// assert_eq!(category.directory(), "data");
    /// ```
    pub fn directory(&self) -> &'static str {
        match self {
            Self::Assets => "assets",
            Self::Data => "data",
        }
    }
}

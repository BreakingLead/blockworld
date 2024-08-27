/// Helper methods for dealing with model identifiers.
///
/// # Why does this exist?
///
/// Prior to 1.13, model identifiers found in
/// `assets/<namespace>/blockstates/*.json` did not include a prefix like
/// `block/` or `item/` to disambiguate between different types of models.
///
/// Because of this, the `minecraft-assets` API forces the user to always
/// specify which type of model they are trying to reference (note the existence
/// of both [`BlockModel`] and [`ItemModel`] variants in [`ResourceKind`]). This
/// way, the API will work with versions prior to 1.13.
///
/// So this struct is meant to wrap an identifier and extract its model name.
/// See the [`model_name()`] documentation for more information.
///
/// [`ResourceKind`]: crate::api::ResourceKind
/// [`BlockModel`]: crate::api::ResourceKind::BlockModel
/// [`ItemModel`]: crate::api::ResourceKind::ItemModel
/// [`model_name()`]: Self::model_name
#[derive(Debug, Clone, Hash)]
pub struct ModelIdentifier;

impl ModelIdentifier {
    /// Returns the name of the model, stripping the leading path component if
    /// there is one.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// assert_eq!(ModelIdentifier::model_name("stone"), "stone");
    /// assert_eq!(ModelIdentifier::model_name("block/oak_planks"), "oak_planks");
    /// assert_eq!(ModelIdentifier::model_name("item/diamond_hoe"), "diamond_hoe");
    /// ```
    pub fn model_name(id: &str) -> &str {
        Self::slash_position(id)
            .map(|index| &id[index + 1..])
            .unwrap_or_else(|| id)
    }

    pub(crate) fn is_builtin(id: &str) -> bool {
        match Self::slash_position(id) {
            Some(index) => {
                let prefix = &id[..index];
                prefix == "builtin"
            }
            None => false,
        }
    }

    fn slash_position(id: &str) -> Option<usize> {
        id.chars().position(|c| c == '/')
    }
}

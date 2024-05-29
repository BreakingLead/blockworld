use std::{ops::Deref, path::Path};

use serde::de::DeserializeOwned;

use crate::io::assets_reader::{
    api::{
        FileSystemResourceProvider, ModelIdentifier, ResourceIdentifier, ResourceProvider, Result,
    },
    schemas::{BlockStates, Model},
};

/// Top-level API for accessing Minecraft assets.
pub struct AssetPack {
    provider: Box<dyn ResourceProvider>,
}

impl AssetPack {
    /// Returns a new [`AssetPack`] that can read data from the given directory.
    ///
    /// The provided `root_dir` should be the directory that contains the
    /// `assets/` and/or `data/` directories.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use minecraft_assets::api::AssetPack;
    ///
    /// let assets = AssetPack::at_path("~/.minecraft/");
    ///
    /// // Load the block states for `oak_planks`
    /// let states = assets.load_blockstates("oak_planks").unwrap();
    /// let variants = states.variants().unwrap();
    ///
    /// assert_eq!(variants.len(), 1);
    ///
    /// let model_properties = &variants[""].models()[0];
    /// assert_eq!(model_properties.model, "block/oak_planks");
    /// ```
    pub fn at_path(root_dir: impl AsRef<Path>) -> Self {
        let provider = FileSystemResourceProvider::new(root_dir);
        Self {
            provider: Box::new(provider),
        }
    }

    /// Returns a new [`AssetPack`] that uses the given [`ResourceProvider`].
    pub fn new<P>(provider: P) -> Self
    where
        P: ResourceProvider + 'static,
    {
        Self {
            provider: Box::new(provider),
        }
    }

    /// Loads the [`BlockStates`] of the block with the provided id.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let states = assets.load_blockstates("stone");
    /// let states = assets.load_blockstates("minecraft:dirt");
    /// ```
    pub fn load_blockstates(&self, block_id: &str) -> Result<BlockStates> {
        self.load_resource(&ResourceIdentifier::blockstates(block_id))
    }

    /// Loads the block [`Model`] identified by the given name or path.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let model = assets.load_block_model("stone");
    /// let model = assets.load_block_model("block/dirt");
    /// ```
    pub fn load_block_model(&self, model: &str) -> Result<Model> {
        self.load_resource(&ResourceIdentifier::block_model(model))
    }

    /// Loads the block [`Model`] identified by the given name or path, as well
    /// as all of its parents and ancestors.
    ///
    /// The models are returned as a list, with the first element being the
    /// model that was originally requested, the next element being its parent,
    /// and so on with the last element being the topmost parent.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let models = assets.load_block_model_recursive("block/cube_all").unwrap();
    ///
    /// let expected = vec![
    ///     assets.load_block_model("block/cube_all").unwrap(),
    ///     assets.load_block_model("block/cube").unwrap(),
    ///     assets.load_block_model("block/block").unwrap(),
    /// ];
    /// assert_eq!(models, expected);
    /// ```
    pub fn load_block_model_recursive(&self, model: &str) -> Result<Vec<Model>> {
        self.load_model_recursive(&ResourceIdentifier::block_model(model))
    }

    /// Loads the item [`Model`] identified by the given name or path.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let model = assets.load_item_model("compass");
    /// let model = assets.load_item_model("item/diamond_hoe");
    /// ```
    pub fn load_item_model(&self, model: &str) -> Result<Model> {
        self.load_resource(&ResourceIdentifier::item_model(model))
    }

    /// Loads the item [`Model`] identified by the given name or path, as well
    /// as all of its parents and ancestors.
    ///
    /// The models are returned as a list, with the first element being the
    /// model that was originally requested, the next element being its parent,
    /// and so on with the last element being the topmost parent.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use minecraft_assets::api::*;
    /// # let assets = AssetPack::at_path("foo");
    /// let models = assets.load_item_model_recursive("item/diamond_hoe").unwrap();
    ///
    /// let expected = vec![
    ///     assets.load_item_model("item/diamond_hoe").unwrap(),
    ///     assets.load_item_model("item/handheld").unwrap(),
    ///     assets.load_item_model("item/generated").unwrap(),
    /// ];
    /// assert_eq!(models, expected);
    /// ```
    pub fn load_item_model_recursive(&self, model: &str) -> Result<Vec<Model>> {
        self.load_model_recursive(&ResourceIdentifier::item_model(model))
    }

    fn load_resource<T>(&self, resource: &ResourceIdentifier) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let bytes = self.provider.load_resource(resource)?;
        Ok(serde_json::from_reader(&bytes[..])?)
    }

    fn load_model_recursive(&self, resource: &ResourceIdentifier) -> Result<Vec<Model>> {
        let mut models = Vec::new();

        Self::for_each_parent(
            resource.clone(),
            |model| models.push(model),
            |next_id| self.load_resource(next_id),
        )?;

        Ok(models)
    }

    pub(crate) fn for_each_parent<F, L, E>(
        mut current: ResourceIdentifier,
        mut op: F,
        mut load_model: L,
    ) -> Result<(), E>
    where
        F: FnMut(Model),
        L: FnMut(&ResourceIdentifier) -> Result<Model, E>,
    {
        loop {
            let model = load_model(&current)?;

            let parent_owned = model.parent.clone();

            op(model);

            match parent_owned {
                Some(parent) if !ModelIdentifier::is_builtin(&parent) => {
                    //println!("{}", parent.as_str());
                    current = ResourceIdentifier::new_owned(current.kind(), parent);
                }
                _ => break,
            }
        }

        Ok(())
    }
}

impl Deref for AssetPack {
    type Target = dyn ResourceProvider;

    fn deref(&self) -> &Self::Target {
        &*self.provider
    }
}

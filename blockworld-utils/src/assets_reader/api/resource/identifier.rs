use std::{borrow::Cow, fmt, hash::Hash};

#[allow(missing_docs)]
pub const MINECRAFT_NAMESPACE: &str = "minecraft";

use crate::io::assets_reader::api::{ModelIdentifier, ResourceKind};

/// A namespaced, typed resource identifier.
///
/// [`ResourceIdentifiers`] reference blocks, items, entity types, recipes,
/// functions, advancements, tags, and various other objects in Minecraft.
///
/// A valid resource identifier has a format of `"namespace:path"`. If the
/// `namespace` portion is left out, then `"minecraft"` is the implied
/// namespace.
///
/// Some examples:
///
/// ```txt
/// "block/stone"
/// "minecraft:diamond_hoe"
/// "foo:bar/baz"
/// ```
///
/// Read more on the [wiki].
///
/// # Borrowing / Ownership
///
/// To avoid cloning / [`String`] construction when not necessary, this type can
/// either borrow or take ownership of the underlying string.
///
/// By default, no copying or allocating is done. You must call
/// [`to_owned()`][Self::to_owned] to get an owned identifier.
///
/// # Strong Typing
///
/// The string representation of a [`ResourceIdentifier`] does not provide
/// enough information to figure out the full path to a given resource. For
/// example, `"block/kelp"` may refer to any of the following:
/// * `assets/minecraft/models/block/kelp.json`
/// * `assets/minecraft/textures/block/kelp.png`
/// * `assets/minecraft/textures/block/kelp.png.mcmeta`
///
/// Because of this, [`ResourceIdentifier`] requires you to specify a
/// [`ResourceKind`] upon construction. Thus, the identifier is "strongly
/// typed."
///
/// This also solves an issue with how model identifiers are formatted in
/// resource packs prior to 1.13. See the [`ModelIdentifier`] docs for more
/// information.
///
/// [wiki]: <https://minecraft.fandom.com/wiki/Resource_location>
/// [`ResourceIdentifiers`]: ResourceIdentifier
#[derive(Clone)]
pub struct ResourceIdentifier<'a> {
    id: Cow<'a, str>,
    kind: ResourceKind,
}

impl<'a> ResourceIdentifier<'a> {
    /// Constructs a new [`ResourceIdentifier`] from the given type and id.
    ///
    /// The `id` string will be **borrowed**. You can either use [`to_owned()`]
    /// to convert the id to an owned representation, or construct on
    /// directly using [`new_owned()`].
    ///
    /// [`to_owned()`]: Self::to_owned
    /// [`new_owned()`]: Self::new_owned
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::new(ResourceKind::BlockModel, "oak_stairs");
    /// ```
    pub fn new(kind: ResourceKind, id: &'a str) -> Self {
        Self {
            id: Cow::Borrowed(id),
            kind,
        }
    }

    /// Like [`new()`], but returns a [`ResourceIdentifier`] that owns its
    /// internal string.
    ///
    /// [`new()`]: Self::new
    pub fn new_owned(kind: ResourceKind, id: String) -> ResourceIdentifier<'static> {
        ResourceIdentifier {
            id: Cow::Owned(id),
            kind,
        }
    }

    /// Constructs a new [`ResourceIdentifier`] referencing the [`BlockStates`] of
    /// the given block id.
    ///
    /// [`BlockStates`]: ResourceKind::BlockStates
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::blockstates("stone");
    /// let id = ResourceIdentifier::blockstates("minecraft:dirt");
    /// ```
    pub fn blockstates(block_id: &'a str) -> Self {
        Self::new(ResourceKind::BlockStates, block_id)
    }

    /// Constructs a new [`ResourceIdentifier`] referencing the [`BlockModel`] of
    /// the given block id.
    ///
    /// [`BlockModel`]: ResourceKind::BlockModel
    pub fn block_model(block_id: &'a str) -> Self {
        Self::new(ResourceKind::BlockModel, block_id)
    }

    /// Constructs a new [`ResourceIdentifier`] referencing the [`ItemModel`] of
    /// the given item id.
    ///
    /// [`ItemModel`]: ResourceKind::ItemModel
    pub fn item_model(item_id: &'a str) -> Self {
        Self::new(ResourceKind::ItemModel, item_id)
    }

    /// Constructs a new [`ResourceIdentifier`] referencing the [`Texture`]
    /// located at the given path.
    ///
    /// [`Texture`]: ResourceKind::Texture
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::texture("block/stone");
    /// let id = ResourceIdentifier::texture("item/diamond_hoe");
    pub fn texture(path: &'a str) -> Self {
        Self::new(ResourceKind::Texture, path)
    }

    /// Returns the underlying identifier as a string slice.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::blockstates("stone");
    /// assert_eq!(id.as_str(), "stone");
    ///
    /// let id = ResourceIdentifier::blockstates("minecraft:dirt");
    /// assert_eq!(id.as_str(), "minecraft:dirt");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.id
    }

    /// Returns whether or not this resource id includes an explicit
    /// namespace.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::blockstates("foo:bar");
    /// assert!(id.has_namespace());
    ///
    /// let id = ResourceIdentifier::blockstates("bar");
    /// assert!(!id.has_namespace());
    /// ```
    pub fn has_namespace(&self) -> bool {
        self.colon_position().is_some()
    }

    /// Returns the namespace portion of the resource identifier, or
    /// `"minecraft"` if it does not have an explicit namespace.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::blockstates("foo:bar");
    /// assert_eq!(id.namespace(), "foo");
    ///
    /// let id = ResourceIdentifier::blockstates("bar");
    /// assert_eq!(id.namespace(), "minecraft");
    ///
    /// let id = ResourceIdentifier::blockstates(":bar");
    /// assert_eq!(id.namespace(), "");
    /// ```
    pub fn namespace(&self) -> &str {
        self.colon_position()
            .map(|index| &self.id[..index])
            .unwrap_or_else(|| MINECRAFT_NAMESPACE)
    }

    /// Returns the path portion of the resource id.
    ///
    /// # Note on Models
    ///
    /// For [`BlockModel`] or [`ItemModel`] resources, the name will **not**
    /// include any leading prefix like `block/` or `item/`. See the
    /// [`ModelIdentifier`] documentation for more information.
    ///
    /// [`BlockModel`]: ResourceKind::BlockModel
    /// [`ItemModel`]: ResourceKind::ItemModel
    pub fn path(&self) -> &str {
        if self.is_model() {
            ModelIdentifier::model_name(self.raw_path())
        } else {
            self.raw_path()
        }
    }

    fn raw_path(&self) -> &str {
        self.colon_position()
            .map(|index| &self.id[index + 1..])
            .unwrap_or_else(|| &self.id)
    }

    /// Returns what kind of resource is referenced by this id.
    pub fn kind(&self) -> ResourceKind {
        self.kind
    }

    /// Returns true if the resource id refers to a built-in resource.
    ///
    /// If `true`, then there is no corresponding file that contains the
    /// resource.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::item_model("builtin/generated");
    /// assert!(id.is_builtin());
    /// ```
    pub fn is_builtin(&self) -> bool {
        if self.is_model() {
            ModelIdentifier::is_builtin(&self.id)
        } else {
            false
        }
    }

    /// Returns a new id with a canonical representation (i.e.,
    /// containing an explicit namespace).
    ///
    /// This will involve allocating a new [`String`] if `self` does not already
    /// contain an explicit namespace.
    ///
    /// # Examples
    ///
    /// Prepends the default namespace when one is not present:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::blockstates("stone");
    /// let canonical = id.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "minecraft:stone");
    /// ```
    ///
    /// Performs a shallow copy when a namespace is already present:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::blockstates("foo:bar");
    /// let canonical = id.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "foo:bar");
    ///
    /// // Prove that it was a cheap copy.
    /// assert_eq!(
    ///     id.as_str().as_ptr() as usize,
    ///     canonical.as_str().as_ptr() as usize,
    /// );
    /// ```
    ///
    /// Prepends `block/` or `item/` for models if missing:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let id = ResourceIdentifier::item_model("minecraft:diamond_hoe");
    /// let canonical = id.to_canonical();
    ///
    /// assert_eq!(canonical.as_str(), "minecraft:item/diamond_hoe");
    /// ```
    pub fn to_canonical(&self) -> ResourceIdentifier<'a> {
        if self.has_namespace()
            && (!self.is_model()
                || self.path().starts_with("item/")
                || self.path().starts_with("block/"))
        {
            self.clone()
        } else {
            let namespace = self.namespace();
            let path = self.canonical_path();

            let canonical = format!("{}:{}", namespace, path);
            Self {
                id: Cow::Owned(canonical),
                kind: self.kind,
            }
        }
    }

    fn canonical_path(&self) -> Cow<'_, str> {
        match self.kind {
            ResourceKind::BlockModel if !self.path().starts_with("block/") => {
                Cow::Owned(format!("block/{}", self.path()))
            }
            ResourceKind::ItemModel if !self.path().starts_with("item/") => {
                Cow::Owned(format!("item/{}", self.path()))
            }
            _ => Cow::Borrowed(self.path()),
        }
    }

    /// Returns a new [`ResourceIdentifier`] that owns the underlying string.
    ///
    /// This is useful for, e.g., storing the id in a data structure or
    /// passing it to another thread.
    ///
    /// By default, all `ResourceIdentifier`s borrow the string they are
    /// constructed with, so no copying will occur unless you call this
    /// function.
    ///
    /// # Examples
    ///
    /// Constructing an id using [`From`] simply borrows the data:
    ///
    /// ```compile_fail
    /// # use minecraft_assets::api::*;
    /// let string = String::new("my:resource");
    ///
    /// let id = ResourceIdentifier::from(&string);
    ///
    /// // Location borrows data from `string`, cannot be sent across threads.
    /// std::thread::spawn(move || println!("{}", id));
    /// ```
    ///
    /// Calling [`to_owned()`][Self::to_owned] on the id allows it to be
    /// sent to the thread:
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let string = "my:resource".to_string();
    ///
    /// let id = ResourceIdentifier::blockstates(&string);
    /// let id = id.to_owned();
    ///
    /// std::thread::spawn(move || println!("{}", id));
    /// ```
    pub fn to_owned(&self) -> ResourceIdentifier<'static> {
        ResourceIdentifier {
            id: Cow::Owned(self.id.clone().into_owned()),
            kind: self.kind,
        }
    }

    pub(crate) fn is_model(&self) -> bool {
        matches!(
            self.kind,
            ResourceKind::BlockModel | ResourceKind::ItemModel
        )
    }

    fn colon_position(&self) -> Option<usize> {
        self.id.chars().position(|c| c == ':')
    }
}

impl<'a> PartialEq for ResourceIdentifier<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.namespace() == other.namespace()
            && self.path() == other.path()
    }
}

impl<'a> Eq for ResourceIdentifier<'a> {}

impl<'a> Hash for ResourceIdentifier<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.namespace().hash(state);
        self.path().hash(state);
        self.kind.hash(state);
    }
}

impl<'a> AsRef<str> for ResourceIdentifier<'a> {
    fn as_ref(&self) -> &str {
        &self.id
    }
}

impl<'a> fmt::Debug for ResourceIdentifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = format!("{:?}", self.kind);
        write!(f, "{}({:?})", kind, &self.id)
    }
}

impl<'a> fmt::Display for ResourceIdentifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical().as_str())
    }
}

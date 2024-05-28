//! Serde-(de)serializable data types for
//! `assets/<namespace>/models/{block,item}/*.json`.
//!
//! Start here: [`Model`].
//!
//! See <https://minecraft.fandom.com/wiki/Model#Block_models>.

use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

/// A block or item model as stored in the
/// `assets/<namespace>/models/{block,item}/` directories.
///
/// See also the corresponding section of the [wiki page]
///
/// [wiki page]: <https://minecraft.fandom.com/wiki/Model#Block_models>
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
pub struct Model {
    /// Specifies that this model should inherit fields from the model at the
    /// given [resource location]. If both `parent` and `elements` are set, the
    /// `elements` field overrides the `elements` field from the parent model.
    ///
    /// For an item model, this can be set to a couple builtin values:
    ///
    /// * `"item/generated"`, to use a model that is created out of the item's
    ///   icon.
    ///
    /// * `"builtin/entity"`, to load a model from an entity file.
    ///   * As you cannot specify the entity, this does not work for all items
    ///     (only for chests, ender chests, mob heads, shields, banners and
    ///     tridents).
    ///
    /// [resource location]: <https://minecraft.fandom.com/wiki/Model#File_path>
    pub parent: Option<String>,

    /// Contains the different places where item models are displayed in
    /// different views.
    pub display: Option<Display>,

    /// Contains the textures of the model.
    pub textures: Option<Textures>,

    /// Contains all the elements of the model.
    ///
    /// If both `parent` and `elements` are set, the `elements` tag overrides
    /// the `elements` tag from the previous model.
    pub elements: Option<Vec<Element>>,

    /// Whether to use ambient occlusion (`true` - default), or not (`false`).
    ///
    /// **Applies only to block models.**
    #[serde(rename = "ambientocclusion")]
    pub ambient_occlusion: Option<bool>,

    /// Specifies how to shade the model in the GUI.
    ///
    /// Can be `front` or `side`. If set to `side`, the model is rendered like a
    /// block. If set to `front`, model is shaded like a flat item. Defaults to
    /// `side`.
    ///
    /// **Applies only to item models.**[^1]
    ///
    /// [^1]: In versions >= 1.16.2, it appears that `block/block.json` also has
    ///     this field set.
    #[serde(rename = "gui_light")]
    pub gui_light_mode: Option<GuiLightMode>,

    /// Specifies cases in which a different model should be used based on item
    /// tags.
    ///
    /// All cases are evaluated in order from top to bottom and last predicate
    /// that matches overrides. However, overrides are ignored if it has been
    /// already overridden once, for example this avoids recursion on overriding
    /// to the same model.
    ///
    /// **Applies only to item models.**
    pub overrides: Option<Vec<OverrideCase>>,
}

/// Specifies how a [`Model`] is displayed in different views.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
pub struct Display {
    /// How the model is displayed when held in the right hand in third-person
    /// view.
    pub thirdperson_righthand: Option<Transform>,

    /// How the model is displayed when held in the left hand in third-person
    /// view.
    pub thirdperson_lefthand: Option<Transform>,

    /// How the model is displayed when held in the right hand in first-person
    /// view.
    pub firstperson_righthand: Option<Transform>,

    /// How the model is displayed when held in the left hand in first-person
    /// view.
    pub firstperson_lefthand: Option<Transform>,

    /// How the model is displayed in the GUI (e.g., in the inventory).
    pub gui: Option<Transform>,

    /// How the model is displayed when worn on the player's head.
    pub head: Option<Transform>,

    /// How the model is displayed when on the ground.
    pub ground: Option<Transform>,

    /// How the model is displayed in an item frame.
    pub fixed: Option<Transform>,
}

/// Specifies the position, rotation, and scale at which a model is displayed.
///
/// Note that translations are applied to the model before rotations.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Transform {
    /// Specifies the rotation of the model in degrees according to the scheme
    /// `[x, y, z]`.
    #[serde(default = "Transform::zeros")]
    pub rotation: [f32; 3],

    /// Specifies the position of the model according to the scheme `[x, y, z]`.
    ///
    /// The unit of distance is **1/16th of a block** (0.0625 meters).
    ///
    /// The values should be clamped between -80 and 80.
    #[serde(default = "Transform::zeros")]
    pub translation: [f32; 3],

    /// Specifies the scale of the model according to the scheme `[x, y, z]`.
    ///
    /// If the value is greater than 4, it is displayed as 4.
    #[serde(default = "Transform::ones")]
    pub scale: [f32; 3],
}

impl Transform {
    pub(crate) const fn zeros() -> [f32; 3] {
        [0.0; 3]
    }

    pub(crate) const fn ones() -> [f32; 3] {
        [1.0; 3]
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: [0.0, 0.0, 0.0],
            translation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

/// Specifies the [`Texture`]s of a [`Model`].
///
/// ## Texture Variables
///
/// A model's textures are specified as a set of named **texture variables**.
/// This allows the value of one texture variable to be set to the value of
/// another via reference, e.g., `"top": "#bottom"`.
///
/// ## Builtin Texture Variables
///
/// * **`particle`**
///   * What texture to load particles from.
///   * This texture is used if you are in a nether portal.
///   * Also used for water and lava's still textures.
///   * Applies to block and item models.
///
/// * **`layerN`**
///   * Used to specify the icon of the item used in the inventory.
///   * There can be more than just one layer (e.g. for spawn eggs), but the
///     amount of possible layers is hardcoded for each item.
///   * Works only in combination with `"item/generated"`.
///   * Applies to item models.
///
/// ## Example
///
/// `block/cross.json` is the common parent of all saplings, and specifies that
/// the `particle` texture variable should take on the value of the `cross`
/// texture variable:
///
/// ```json
/// {
///     "textures": {
///         "particle": "#cross"
///     },
///     ...
/// }
/// ```
///
/// `block/oak_sapling.json` specifies a concrete location for the `cross`
/// texture variable:
///
/// ```json
/// {
///     "parent": "block/cross",
///     "textures": {
///         "cross": "block/oak_sapling"
///     }
/// }
/// ```
///
/// [resource location]: <https://minecraft.fandom.com/wiki/Model#File_path>
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Textures {
    /// The values of all texture variables by name.
    #[serde(flatten)]
    pub variables: HashMap<String, Texture>,
}

impl Textures {
    /// Attempts to resolve each of the texture variables in `self` using the
    /// values present in `other`.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::schemas::models::*;
    /// use maplit::hashmap;
    ///
    /// let mut textures = Textures::from(hashmap! {
    ///     "foo" => "#foobar",
    ///     "bar" => "#barvar"
    /// });
    ///
    /// textures.resolve(&Textures::from(hashmap! {
    ///     "barvar" => "herobrine",
    /// }));
    ///
    /// let expected = Textures::from(hashmap! {
    ///     "foo" => "#foobar",
    ///     "bar" => "herobrine",
    /// });
    ///
    /// assert_eq!(textures, expected);
    /// ```
    pub fn resolve(&mut self, other: &Self) {
        for texture in self.values_mut() {
            if let Some(substitution) = texture.resolve(other) {
                *texture = Texture::from(substitution);
            }
        }
    }

    /// Merges the values from `other` into `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::schemas::models::*;
    /// use maplit::hashmap;
    ///
    /// let mut textures = Textures::from(hashmap! {
    ///     "foo" => "#foobar",
    ///     "bar" => "#barvar"
    /// });
    ///
    /// textures.merge(Textures::from(hashmap! {
    ///     "foo" => "fooey",
    ///     "creeper" => "aw man"
    /// }));
    ///
    /// let expected = Textures::from(hashmap! {
    ///     "foo" => "fooey",
    ///     "creeper" => "aw man",
    ///     "bar" => "#barvar"
    /// });
    ///
    /// assert_eq!(textures, expected);
    /// ```
    pub fn merge(&mut self, other: Self) {
        for (name, texture) in other.variables.into_iter() {
            //println!("inserting: {:?}", (&name, &texture));
            self.insert(name, texture);
        }
    }
}

impl<K, V> From<HashMap<K, V>> for Textures
where
    K: Into<String>,
    V: Into<Texture>,
{
    fn from(source: HashMap<K, V>) -> Self {
        let variables = source
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self { variables }
    }
}

impl Deref for Textures {
    type Target = HashMap<String, Texture>;

    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}

impl DerefMut for Textures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variables
    }
}

/// The value of a [texture variable] in the [`Textures`] map.
///
/// The string value will either specify a [`location`] to load the texture from
/// or a [`reference`] to another texture variable to take its value from.
///
/// [texture variable]: Textures#texture-variables
/// [`location`]: Self::location
/// [`reference`]: Self::reference
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Texture(pub String);

impl Texture {
    /// Returns the [resource location] of the texture, or `None` if the texture
    /// should instead take on the value of another texture variable.
    ///
    /// [resource location]: <https://minecraft.fandom.com/wiki/Model#File_path>
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::schemas::models::*;
    /// let texture = Texture::from("texture/location");
    /// assert_eq!(texture.location(), Some("texture/location"));
    ///
    /// let texture = Texture::from("#another_var");
    /// assert_eq!(texture.location(), None);
    pub fn location(&self) -> Option<&str> {
        if self.0.starts_with('#') {
            None
        } else {
            Some(&self.0[..])
        }
    }

    /// Returns the name of the texture variable from which this texture should
    /// get its value, or `None` if the texture should be loaded from a
    /// resource.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::schemas::models::*;
    /// let texture = Texture::from("texture/location");
    /// assert_eq!(texture.reference(), None);
    ///
    /// let texture = Texture::from("#another_var");
    /// assert_eq!(texture.reference(), Some("another_var"));
    /// ```
    pub fn reference(&self) -> Option<&str> {
        if self.0.starts_with('#') {
            Some(&self.0[1..])
        } else {
            None
        }
    }

    /// Resolves this texture value using the variables present in `other`, or
    /// returns `None` if:
    /// * This texture value not reference another texture variable, or
    /// * There is no variable in `other` that matches
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::schemas::models::*;
    /// use maplit::hashmap;
    ///
    /// let substitutions = Textures::from(hashmap! {
    ///     "foo" => "textures/foo",
    ///     "bar" => "#another_var",
    /// });
    ///
    /// let texture = Texture::from("#foo");
    /// assert_eq!(texture.resolve(&substitutions), Some("textures/foo"));
    ///
    /// let texture = Texture::from("#bar");
    /// assert_eq!(texture.resolve(&substitutions), Some("#another_var"));
    ///
    /// let texture = Texture::from("#not_found");
    /// assert_eq!(texture.resolve(&substitutions), None);
    ///
    /// let texture = Texture::from("not_a_reference");
    /// assert_eq!(texture.resolve(&substitutions), None);
    /// ```
    pub fn resolve<'a>(&'a self, substitutions: &'a Textures) -> Option<&'a str> {
        if let Some(reference) = self.reference() {
            if let Some(substitution) = substitutions.get(reference) {
                return Some(&substitution.0);
            }
        }
        None
    }
}

impl From<String> for Texture {
    fn from(source: String) -> Self {
        Self(source)
    }
}

impl<'a> From<&'a str> for Texture {
    fn from(source: &'a str) -> Self {
        Self(String::from(source))
    }
}

/// A single, cube-shaped element of a [`Model`]'s geometry.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Element {
    /// Start point of a cuboid according to the scheme `[x, y, z]`.
    ///
    /// Values must be between -16 and 32.
    pub from: [f32; 3],

    /// Stop point of a cuboid according to the scheme `[x, y, z]`.
    ///
    /// Values must be between -16 and 32.
    pub to: [f32; 3],

    /// Holds all the faces of the cuboid. If a face is left out, it does not
    /// render.
    pub faces: HashMap<BlockFace, ElementFace>,

    /// The rotation of the element
    #[serde(default)]
    pub rotation: ElementRotation,

    /// Specifies if shadows are rendered (`true` - default), or not (`false`).
    #[serde(default = "Element::default_shade")]
    pub shade: bool,
}

impl Element {
    pub(crate) const fn default_shade() -> bool {
        true
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            from: [0.0, 0.0, 0.0],
            to: [16.0, 16.0, 16.0],
            faces: Default::default(),
            rotation: Default::default(),
            shade: Self::default_shade(),
        }
    }
}

/// Specifies the rotation of an [`Element`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ElementRotation {
    /// Sets the center of the rotation according to the scheme `[x, y, z]`.
    pub origin: [f32; 3],

    /// Specifies the direction of rotation.
    pub axis: Axis,

    /// Specifies the angle of rotation.
    ///
    /// Can be 45 through -45 degrees in 22.5 degree increments.
    pub angle: f32,

    /// Specifies whether or not to scale the faces across the whole block.
    ///
    /// Defaults to `false`.
    #[serde(default = "ElementRotation::default_rescale")]
    pub rescale: bool,
}

impl ElementRotation {
    pub(crate) const fn default_rescale() -> bool {
        false
    }
}

impl Default for ElementRotation {
    fn default() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            axis: Axis::X,
            angle: 0.0,
            rescale: Self::default_rescale(),
        }
    }
}

/// Specifies the details of a single face in a cuboid [`Element`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ElementFace {
    /// Defines the area of the image that should be sampled for this texture.
    ///
    /// The UV coordinates are specified as `[x1, y1, x2, y2]`.
    ///
    /// UV is optional, and if not supplied it defaults to values equal to the
    /// xyz position of the element.
    ///
    /// The texture behavior is inconsistent if UV extends below 0 or above 16.
    /// If the numbers of `x1` and `x2` are swapped (e.g. from `0, 0, 16, 16` to
    /// `16, 0, 0, 16`), the texture flips.
    pub uv: Option<[f32; 4]>,

    /// Specifies the texture as [texture variable] prepended with a `#`.
    ///
    /// [texture variable]: Textures#texture-variables
    pub texture: Texture,

    /// Specifies whether a face does not need to be rendered when there is a
    /// block touching it in the specified position.
    ///
    /// The position can be: `down`, `up`, `north`, `south`, `west`, or `east`.
    ///
    /// It also determines the side of the block to use the light level from for
    /// lighting the face, and if unset, defaults to the side.
    ///
    /// `bottom` may also be used in the latest versions instead of `down`,
    /// despite appearing only once in the actual game assets.
    #[serde(rename = "cullface")]
    pub cull_face: Option<BlockFace>,

    /// Rotates the texture by the specified number of degrees.
    ///
    /// Can be `0`, `90`, `180`, or `270`. Defaults to `0`. Rotation does not
    /// affect which part of the texture is used. Instead, it amounts to a
    /// permutation of the selected texture vertexes (selected implicitly, or
    /// explicitly though `uv`).
    #[serde(default = "ElementFace::default_rotation")]
    pub rotation: u32,

    /// Determines whether to tint the texture using a hardcoded tint index.
    ///
    /// The default value, `-1`, indicates not to use the tint. Any other number
    /// is provided to BlockColors to get the tint value corresponding to that
    /// index. However, most blocks do not have a tint value defined (in which
    /// case white is used). Furthermore, no vanilla block currently uses
    /// multiple tint values, and thus the tint index value is ignored (as long
    /// as it is set to something other than `-1`); it could be used for modded
    /// blocks that need multiple distinct tint values in the same block though.
    #[serde(rename = "tintindex", default = "ElementFace::default_tint_index")]
    pub tint_index: i32,
}

impl ElementFace {
    pub(crate) const fn default_rotation() -> u32 {
        0
    }

    pub(crate) const fn default_tint_index() -> i32 {
        -1
    }
}

impl Default for ElementFace {
    fn default() -> Self {
        Self {
            uv: Default::default(),
            texture: Default::default(),
            cull_face: Default::default(),
            rotation: Self::default_rotation(),
            tint_index: Self::default_tint_index(),
        }
    }
}

/// One possible case in which an item's [`Model`] should be overridden.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OverrideCase {
    /// Specifies when this override should be active.
    ///
    /// See the [wiki page] for a list of possible item predicates.
    ///
    /// [wiki page]: <https://minecraft.fandom.com/wiki/Model#Item_predicates>
    pub predicate: HashMap<String, PredicateValue>,

    /// The path to the model to use if the case is met, in form of a [resource
    /// location].
    ///
    /// [resource location]: <https://minecraft.fandom.com/wiki/Model#File_path>
    pub model: String,
}

/// The value for an item tag specified in a predicate in an [`OverrideCase`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum PredicateValue {
    Int(u32),
    Float(f32),
}

/// The two possible ways to shade a model in the UI.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum GuiLightMode {
    /// Shade the model like a block.
    Side,

    /// Shade the model like a flat item.
    Front,
}

/// The three possible axes in 3D space.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// The six possible faces of a cuboid.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
#[repr(u8)]
pub enum BlockFace {
    // The format accepts two possible names for `"down"`.
    #[serde(alias = "bottom")]
    Down,
    Up,
    North,
    South,
    West,
    East,
}

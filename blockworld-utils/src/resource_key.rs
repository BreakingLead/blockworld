//! What's a resource key?

use crate::resource_location::ResourceLocation;

/// ResourceKeys combine a registry id with a registry name.
/// An example would be a registry key with the registry id `minecraft:item` and the registry name `minecraft:diamond_sword.`
///
/// Unlike a [`ResourceLocation`], ResourceKeys actually refer to a unique element, thus being able to clearly identify an element.
/// They are most commonly used in contexts where many different registries come in contact with one another.
/// A common use case are datapacks, especially worldgen.
pub struct ResourceKey {
    pub registry_name: ResourceLocation,
    pub location: ResourceLocation,
}

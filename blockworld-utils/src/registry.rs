use crate::resource_location::ResourceLocation;

pub trait Registry<V> {
    fn get(name: &ResourceLocation) -> Option<&V>;
    /// From value to key.
    fn get_key(&self, value: &V) -> Option<ResourceLocation>;
}

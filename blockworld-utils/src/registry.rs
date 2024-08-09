use std::collections::HashMap;

use crate::resource_location::ResourceLocation;

pub struct Registry<V> {
    data: HashMap<ResourceLocation, V>,
}

impl<V> Registry<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn register(&mut self, name: ResourceLocation, value: V) {
        self.data.insert(name, value);
    }

    fn get(&self, name: &ResourceLocation) -> Option<&V> {
        self.data.get(name)
    }
    /// From value to key.
    fn get_key(&self, value: &V) -> Option<ResourceLocation> {
        todo!()
    }
}

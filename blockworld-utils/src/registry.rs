use std::collections::HashMap;

use bimap::BiMap;

use crate::{resource::resource_location::HasResourceLocation, ResourceLocation};

pub struct Registry<V: HasResourceLocation> {
    // first time i thought V shouldn't store a ResourceLocation,
    // but I'm wrong
    // I need the ResourceLocation as a index of a registry
    // so V has a ResourceLocation and I need store it again in the key
    // like this
    // "air": {id :"air", ...}
    // dumb idea probably
    data: HashMap<ResourceLocation, V>,
    id_bimap: BiMap<u32, ResourceLocation>,
    counter: u32,
}

impl<V: HasResourceLocation> Registry<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            id_bimap: BiMap::new(),
            counter: 0,
        }
    }

    pub fn register(&mut self, value: V) {
        self.id_bimap.insert(self.counter, value.get_id());
        self.data.insert(value.get_id(), value);
        self.counter += 1;
    }

    pub fn get(&self, name: &ResourceLocation) -> Option<&V> {
        self.data.get(name)
    }

    pub fn number_id_to_name(&self, id: u32) -> Option<&ResourceLocation> {
        self.id_bimap.get_by_left(&id)
    }

    pub fn name_to_number_id(&self, id: &ResourceLocation) -> u32 {
        *self.id_bimap.get_by_right(id).unwrap_or(&0)
    }

    pub fn get_with_number_id(&self, id: &ResourceLocation) -> (u32, Option<&V>) {
        let number_id = self.name_to_number_id(id);
        (number_id, self.get(id))
    }
}

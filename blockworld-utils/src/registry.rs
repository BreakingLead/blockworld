//! Generic bidirectional registry (Identifier <-> numeric ID).
//!
//! Equivalent to Minecraft's built-in registry system where every
//! block/item/entity gets both a string name (`blockworld:stone`) and
//! a compact numeric ID (`1`) for efficient storage and networking.

use std::collections::HashMap;

use bimap::BiMap;

use crate::{resource::resource_location::HasIdentifier, Identifier};

/// A registry mapping `Identifier` ↔ `u32`, plus `Identifier → V` lookup.
///
/// `V` must implement `HasIdentifier` (i.e. carry its own name).
/// This creates a slight redundancy — the key in `data` is the same
/// string stored inside the value — but it keeps the API simple.
pub struct Registry<V: HasIdentifier> {
    /// `Identifier → value` lookup.
    data: HashMap<Identifier, V>,
    /// Bidirectional mapping between numeric IDs and identifiers.
    id_bimap: BiMap<u32, Identifier>,
    counter: u32,
}

impl<V: HasIdentifier> Registry<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            id_bimap: BiMap::new(),
            counter: 0,
        }
    }

    /// Assign the next available numeric ID and register the value.
    pub fn register(&mut self, value: V) {
        self.id_bimap.insert(self.counter, value.get_id());
        self.data.insert(value.get_id(), value);
        self.counter += 1;
    }

    /// Look up a value by its string identifier.
    pub fn get(&self, name: &Identifier) -> Option<&V> {
        self.data.get(name)
    }

    /// Convert numeric ID back to identifier.
    /// Returns `None` if the ID is unregistered (shouldn't happen with valid data).
    pub fn number_id_to_name(&self, id: u32) -> Option<&Identifier> {
        self.id_bimap.get_by_left(&id)
    }

    /// Convert identifier to numeric ID.
    /// Returns `0` for unregistered identifiers (air by convention).
    pub fn name_to_number_id(&self, id: &Identifier) -> u32 {
        *self.id_bimap.get_by_right(id).unwrap_or(&0)
    }

    /// Convenience: get both the numeric ID and the value in one call.
    pub fn get_with_number_id(&self, id: &Identifier) -> (u32, Option<&V>) {
        let number_id = self.name_to_number_id(id);
        (number_id, self.get(id))
    }
}

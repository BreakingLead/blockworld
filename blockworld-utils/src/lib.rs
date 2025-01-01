use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

mod constants;
mod registry;
mod resource;

pub use constants::*;
pub use registry::Registry;
pub use resource::resource_location::HasResourceLocation;
pub use resource::resource_location::ResourceLocation;

pub type AM<T> = Arc<Mutex<T>>;
pub type RR<T> = Rc<RefCell<T>>;
pub type OAM<T> = Option<AM<T>>;
pub type ORR<T> = Option<RR<T>>;

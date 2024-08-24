use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub mod constants;
pub mod registry;
pub mod resource_key;
pub mod resource_location;
pub mod text;

pub use resource_location::ResourceLocation;

pub type AM<T> = Arc<Mutex<T>>;
pub type RR<T> = Rc<RefCell<T>>;
pub type OAM<T> = Option<AM<T>>;
pub type ORR<T> = Option<RR<T>>;

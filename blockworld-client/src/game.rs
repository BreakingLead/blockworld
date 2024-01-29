//! Game logic of blockworld


/// A coordinate of a block. When it's used as a position of a vertex,
/// it represents the North(-Z) West(-X) Down(-Y) corner of the block.
/// For example, 0,0,0 represents the cube [0,0,0] to [1,1,1]
pub type BlockCoordinate = [i32;3];
pub type Domain = String;
pub type URL = String;

const BLOCKWORLD: &str = "blockworld";

pub struct ResourceLocation {
    domain: Domain,
    url: URL,
}

impl ResourceLocation {
    /// Panics on path is not formatted correctly.
    fn new(domain: Domain, path: URL) -> Self{
        let (domain,url) = path.split_once(':').unwrap();
        ResourceLocation {
            domain: domain.to_string(),
            url: url.to_string(),
        }
    }
}

mod chunk;
mod block;
mod register;
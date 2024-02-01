use std::collections::HashMap;

pub const SUBCHUNK_Y_SIZE: usize = 16;
pub const CHUNK_Y_SIZE: usize = SUBCHUNK_Y_SIZE * 16;
pub const CHUNK_Z_SIZE: usize = 16;
pub const CHUNK_X_SIZE: usize = 16;

pub const CHUNK_SIZE: usize = CHUNK_X_SIZE * CHUNK_Y_SIZE * CHUNK_Z_SIZE;
pub const CHUNK_LAYER_SIZE: usize = CHUNK_X_SIZE * CHUNK_Z_SIZE;


/// Chunk's block array is flattened, so this function will gill the xyz of block which is corresponding to the index of the chunk(0-65535).
pub fn from_index_to_relative_xyz(index: usize) -> (i32,i32,i32) {
    let y = index / CHUNK_LAYER_SIZE;
    let l = index % CHUNK_LAYER_SIZE;
    let z = l / CHUNK_X_SIZE;
    let x = index % CHUNK_X_SIZE;
    (x as _,y as _,z as _)
}

pub fn from_relative_xyz_to_index(x:usize,y:usize,z:usize) -> usize{
    y * CHUNK_LAYER_SIZE + z * CHUNK_X_SIZE + x
}

/*
struct ChunkWithRenderData {

}
*/

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn relative_xyz() {
        let k = from_index_to_relative_xyz(0);
        assert_eq!(k,(0,0,0));
        let k = from_index_to_relative_xyz(255);
        assert_eq!(k,(15,0,15));
        let k = from_index_to_relative_xyz(256);
        assert_eq!(k,(0,1,0));
        let k = from_index_to_relative_xyz(257);
        assert_eq!(k,(1,1,0));
        let k = from_index_to_relative_xyz(258);
        assert_eq!(k,(2,1,0));
        let k = from_index_to_relative_xyz(272);
        assert_eq!(k,(0,1,1));

        let m = from_relative_xyz_to_index(0,0,0);
        assert_eq!(m, 0);
        let m = from_relative_xyz_to_index(15,0,15);
        assert_eq!(m, 255);
        let m = from_relative_xyz_to_index(0,1,0);
        assert_eq!(m, 256);
    }
}
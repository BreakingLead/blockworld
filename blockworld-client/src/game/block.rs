use crate::render::{self, vertex::Vertex};

use super::{BlockCoordinate, Domain};


#[derive(Debug)]
struct BlockState;


/// A data struct that represents a block
/// It's not actually a block in the world, it's just some data to be rendered.
#[derive(Debug)]
pub struct RenderableBlock {
    pub position: BlockCoordinate,

    pub faces: [BlockFace;6],
    
    /// For future use
    pub block_state: Option<BlockState>,
}

impl RenderableBlock {
    fn from_pos(pos: BlockCoordinate) -> Self {
        let (x,y,z) = (pos[0],pos[1],pos[2]);
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BlockFaceDirection {
    UP,DOWN,NORTH,EAST,SOUTH,WEST
}

#[derive(Copy, Clone, Debug)]
pub struct BlockFace {
    pub direction: BlockFaceDirection,
    pub vertices: [Vertex ; 4],
}

#[rustfmt::skip]
impl BlockFace {
    /// Vertex arranged in clockwise direction like this:
    /// ```
    /// 1 2
    /// 4 3
    /// ```
    pub fn new(pos: BlockCoordinate, face_dir: BlockFaceDirection) -> Self {
        let (x,y,z) = (pos[0] as f32,pos[1] as f32,pos[2] as f32);
        let dlb = [x  ,y  ,z  ];
        let dlf = [x  ,y  ,z+1.0];
        let drb = [x+1.0,y  ,z  ];
        let drf = [x+1.0,y  ,z+1.0];
        let ulb = [x  ,y+1.0,z  ];
        let ulf = [x  ,y+1.0,z+1.0];
        let urb = [x+1.0,y+1.0,z  ];
        let urf = [x+1.0,y+1.0,z+1.0];
        let vtx = match face_dir{
            BlockFaceDirection::UP     => {
                [
                    Vertex {
                        position: ulb,
                        tex_coords: [0.0,0.0]
                    },
                    Vertex {
                        position: urb,
                        tex_coords: [1.0,0.0]
                    },
                    Vertex {
                        position: urf,
                        tex_coords: [1.0,1.0]
                    },
                    Vertex {
                        position: ulf,
                        tex_coords: [0.0,1.0]
                    },
                ]
            },
            BlockFaceDirection::DOWN   => {
                [
                    Vertex {
                        position: dlf,
                        tex_coords: [0.0,0.0]
                    },
                    Vertex {
                        position: drf,
                        tex_coords: [1.0,0.0]
                    },
                    Vertex {
                        position: drb,
                        tex_coords: [1.0,1.0]
                    },
                    Vertex {
                        position: dlb,
                        tex_coords: [0.0,1.0]
                    },
                ]
            },
            BlockFaceDirection::NORTH  => {
                [
                    Vertex {
                        position: urb,
                        tex_coords: [0.0,0.0]
                    },
                    Vertex {
                        position: ulb,
                        tex_coords: [1.0,0.0]
                    },
                    Vertex {
                        position: dlb,
                        tex_coords: [1.0,1.0]
                    },
                    Vertex {
                        position: drb,
                        tex_coords: [0.0,1.0]
                    },
                ]
            },
            BlockFaceDirection::EAST   => {
                [
                    Vertex {
                        position: urf,
                        tex_coords: [0.0,0.0]
                    },
                    Vertex {
                        position: urb,
                        tex_coords: [1.0,0.0]
                    },
                    Vertex {
                        position: drb,
                        tex_coords: [1.0,1.0]
                    },
                    Vertex {
                        position: drf,
                        tex_coords: [0.0,1.0]
                    },
                ]
            },
            BlockFaceDirection::SOUTH  => {
                [
                    Vertex {
                        position: ulf,
                        tex_coords: [0.0,0.0]
                    },
                    Vertex {
                        position: urf,
                        tex_coords: [1.0,0.0]
                    },
                    Vertex {
                        position: drf,
                        tex_coords: [1.0,1.0]
                    },
                    Vertex {
                        position: dlf,
                        tex_coords: [0.0,1.0]
                    },
                ]
            },
            BlockFaceDirection::WEST   => {
                [
                    Vertex {
                        position: ulb,
                        tex_coords: [0.0,0.0]
                    },
                    Vertex {
                        position: ulf,
                        tex_coords: [1.0,0.0]
                    },
                    Vertex {
                        position: dlf,
                        tex_coords: [1.0,1.0]
                    },
                    Vertex {
                        position: dlb,
                        tex_coords: [0.0,1.0]
                    },
                ]
            },
        };
        Self {
            direction: face_dir,
            vertices: vtx,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_face() {
        let up = BlockFace::new([0,0,0], BlockFaceDirection::UP);
        dbg!(up);
    }
}
use glam::IVec3;

use crate::render::vertex::BlockVertex;


#[derive(Copy, Clone, Debug)]
pub enum FaceDirection {
    UP,DOWN,EAST,SOUTH,WEST,NORTH
}
impl FaceDirection {
    fn get_vertices(self,pos: &IVec3) -> [BlockVertex;4] {
        let (x,y,z) = (pos[0] ,pos[1] ,pos[2] );
        let dlb = [x  ,y  ,z  ];
        let dlf = [x  ,y  ,z+1];
        let drb = [x+1,y  ,z  ];
        let drf = [x+1,y  ,z+1];
        let ulb = [x  ,y+1,z  ];
        let ulf = [x  ,y+1,z+1];
        let urb = [x+1,y+1,z  ];
        let urf = [x+1,y+1,z+1];
        let vtx = match self{
            FaceDirection::UP     => {
                [
                    BlockVertex { position: ulb, tex_coords: [0.0,0.0] },
                    BlockVertex { position: urb, tex_coords: [1.0,0.0] },
                    BlockVertex { position: urf, tex_coords: [1.0,1.0] },
                    BlockVertex { position: ulf, tex_coords: [0.0,1.0] },
                ]
            },
            FaceDirection::DOWN   => {
                [
                    BlockVertex { position: dlf, tex_coords: [0.0,0.0] },
                    BlockVertex { position: drf, tex_coords: [1.0,0.0] },
                    BlockVertex { position: drb, tex_coords: [1.0,1.0] },
                    BlockVertex { position: dlb, tex_coords: [0.0,1.0] },
                ]
            },
            FaceDirection::NORTH  => {
                [
                    BlockVertex { position: urb, tex_coords: [0.0,0.0] },
                    BlockVertex { position: ulb, tex_coords: [1.0,0.0] },
                    BlockVertex { position: dlb, tex_coords: [1.0,1.0] },
                    BlockVertex { position: drb, tex_coords: [0.0,1.0] },
                ]
            },
            FaceDirection::EAST   => {
                [
                    BlockVertex { position: urf, tex_coords: [0.0,0.0] },
                    BlockVertex { position: urb, tex_coords: [1.0,0.0] },
                    BlockVertex { position: drb, tex_coords: [1.0,1.0] },
                    BlockVertex { position: drf, tex_coords: [0.0,1.0] },
                ]
            },
            FaceDirection::SOUTH  => {
                [
                    BlockVertex { position: ulf, tex_coords: [0.0,0.0] },
                    BlockVertex { position: urf, tex_coords: [1.0,0.0] },
                    BlockVertex { position: drf, tex_coords: [1.0,1.0] },
                    BlockVertex { position: dlf, tex_coords: [0.0,1.0] },
                ]
            },
            FaceDirection::WEST   => {
                [
                    BlockVertex { position: ulb, tex_coords: [0.0,0.0] },
                    BlockVertex { position: ulf, tex_coords: [1.0,0.0] },
                    BlockVertex { position: dlf, tex_coords: [1.0,1.0] },
                    BlockVertex { position: dlb, tex_coords: [0.0,1.0] },
                ]
            },
        };
        vtx
    }
}

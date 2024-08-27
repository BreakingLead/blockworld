//! Helper methods and structures for working with cubes.

use super::vertex::TexturedVertex;
use glam::*;

#[repr(u8)]
#[derive(Clone, Copy)]
enum BlockFaceDirection {
    /// X+ (east)
    XP = 0b000001,
    /// Y+ (up)
    YP = 0b000010,
    /// Z+ (south)
    ZP = 0b000100,
    /// X- (west)
    XN = 0b001000,
    /// Y- (down)
    YN = 0b010000,
    /// Z- (north)
    ZN = 0b100000,
}

/// Cube vertices.

impl BlockFaceDirection {
    #[rustfmt::skip]
    pub const VERTICES: &'static [Vec3; 8] = &[
        vec3( 0.5, 0.5,-0.5), // 0
        vec3(-0.5, 0.5,-0.5), // 1
        vec3(-0.5, 0.5, 0.5), // 2
        vec3( 0.5, 0.5, 0.5), // 3
        vec3( 0.5,-0.5,-0.5), // 4
        vec3(-0.5,-0.5,-0.5), // 5
        vec3(-0.5,-0.5, 0.5), // 6
        vec3( 0.5,-0.5, 0.5), // 7
    ];

    ///
    /// ```ignore
    ///
    ///               
    ///            1--------0  
    ///           /  Y+   / |
    ///          /       /  |
    ///         2-------3   |    --> X+          
    ///         |       |   4           
    ///         |  Z+   |  /     
    ///         |       | /
    ///         6-------7
    /// ```
    ///
    pub const QUADS: &'static [[usize; 4]; 6] = &[
        [0, 3, 7, 4], // X+
        [0, 1, 2, 3], // Y+
        [3, 2, 6, 7], // Z+
        [2, 1, 5, 6], // X-
        [7, 6, 5, 4], // Y-
        [1, 0, 4, 5], // Z-
    ];

    /// Generate direction vector.
    fn direction_vector(&self) -> Vec3 {
        match self {
            BlockFaceDirection::XP => Vec3::X,
            BlockFaceDirection::YP => Vec3::Y,
            BlockFaceDirection::ZP => Vec3::Z,
            BlockFaceDirection::XN => -Vec3::X,
            BlockFaceDirection::YN => -Vec3::Y,
            BlockFaceDirection::ZN => -Vec3::Z,
        }
    }

    /// Get the four vectors prependicular to self
    /// and along the crossline of the face in order to move vertices.
    /// return order:
    /// ```ignore
    /// 2 <-- 1
    /// |
    /// v
    /// 3 --> 4
    /// ```
    /// - bias: center of the face.
    fn to_vertices(self, bias: Vec3) -> [Vec3; 4] {
        // ilog means 000001 -> 0
        //            000010 -> 1
        //            000100 -> 2
        // etc.

        Self::QUADS[(self as u8).ilog2() as usize]
            .map(|i| Self::VERTICES[i])
            .map(|v| bias + v)
    }

    pub fn to_quad_mesh(self, quad_center: Vec3, uv_aa: Vec2, uv_bb: Vec2) -> [TexturedVertex; 6] {
        let aa = uv_aa;
        let bb = uv_bb;
        let vecs = self.to_vertices(quad_center);
        [
            TexturedVertex::new(vecs[0], vec2(bb.x, aa.y)),
            TexturedVertex::new(vecs[1], vec2(aa.x, aa.y)),
            TexturedVertex::new(vecs[2], vec2(aa.x, bb.y)),
            TexturedVertex::new(vecs[0], vec2(bb.x, aa.y)),
            TexturedVertex::new(vecs[2], vec2(aa.x, bb.y)),
            TexturedVertex::new(vecs[3], vec2(bb.x, bb.y)),
        ]
    }
}

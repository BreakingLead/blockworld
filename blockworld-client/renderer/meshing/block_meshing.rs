//! Low-level cube face mesh generation.
//!
//! Defines the 8 vertices of a unit cube centered at origin (±0.5),
//! 6 quad definitions (which 4 vertices form each face), and produces
//! 2 triangles per face (6 `TexturedVertex` per face).

use blockworld_server::block::block_face_direction::BlockFaceDirection;
use glam::*;

use crate::renderer::vertex::TexturedVertex;

/// The 8 corners of a unit cube centered at origin.
///
/// ```ignore
///            1--------0
///           /  Y+   / |
///          /       /  |
///         2-------3   |    --> X+
///         |       |   4
///         |  Z+   |  /
///         |       | /
///         6-------7
/// ```
#[rustfmt::skip]
const VERTICES: &'static [Vec3; 8] = &[
    vec3( 0.5, 0.5,-0.5), // 0
    vec3(-0.5, 0.5,-0.5), // 1
    vec3(-0.5, 0.5, 0.5), // 2
    vec3( 0.5, 0.5, 0.5), // 3
    vec3( 0.5,-0.5,-0.5), // 4
    vec3(-0.5,-0.5,-0.5), // 5
    vec3(-0.5,-0.5, 0.5), // 6
    vec3( 0.5,-0.5, 0.5), // 7
];

/// Which 4 vertex indices form each face.
///
/// Ordered counter-clockwise when viewed from outside the cube,
/// so back-face culling works correctly with `FrontFace::Ccw`.
/// Index by `(face as u8).ilog2()`.
#[rustfmt::skip]
const QUADS: &'static [[usize; 4]; 6] = &[
    [0, 3, 7, 4], // X+ (east)
    [0, 1, 2, 3], // Y+ (up)
    [3, 2, 6, 7], // Z+ (south)
    [2, 1, 5, 6], // X- (west)
    [7, 6, 5, 4], // Y- (down)
    [1, 0, 4, 5], // Z- (north)
];

/// Get the four corner positions for a given face, offset by `bias`.
///
/// `(face as u8).ilog2()` maps the bitflag enum value to the QUADS index:
///   XP=0b000001 → ilog2=0, YP=0b000010 → ilog2=1, ZP=0b000100 → ilog2=2, etc.
fn to_vertices(face: BlockFaceDirection, bias: Vec3) -> [Vec3; 4] {
    QUADS[(face as u8).ilog2() as usize]
        .map(|i| VERTICES[i])
        .map(|v| bias + v)
}

/// Generate 6 `TexturedVertex` (2 triangles) for a single block face.
///
/// `quad_center`: world-space center of the face.
/// `uv_aa`, `uv_bb`: texture UV rectangle for this block type.
///
/// Returns vertices in triangle-list order ready for GPU upload.
pub fn to_quad_mesh(
    face: BlockFaceDirection,
    quad_center: Vec3,
    uv_aa: Vec2,
    uv_bb: Vec2,
) -> [TexturedVertex; 6] {
    let aa = uv_aa;
    let bb = uv_bb;
    let vecs = to_vertices(face, quad_center);
    // Two triangles covering the quad: (0,1,2) and (0,2,3)
    [
        TexturedVertex::new(vecs[0], vec2(bb.x, aa.y)),
        TexturedVertex::new(vecs[1], vec2(aa.x, aa.y)),
        TexturedVertex::new(vecs[2], vec2(aa.x, bb.y)),
        TexturedVertex::new(vecs[0], vec2(bb.x, aa.y)),
        TexturedVertex::new(vecs[2], vec2(aa.x, bb.y)),
        TexturedVertex::new(vecs[3], vec2(bb.x, bb.y)),
    ]
}

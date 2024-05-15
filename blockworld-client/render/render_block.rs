use glam::{vec3, Vec2, Vec3};

use super::{texture::AtlasCoordinate, vertex::Vertex};

/// Which axis
pub enum Axis {
    X,
    Y,
    Z,
}

/// XYZ+ or XYZ-
pub enum Sign {
    Pos,
    Neg,
}

/// A enum which indicates the direction of a face of a block.
pub struct AxisDirection {
    axis: Axis,
    sign: Sign,
}

pub const XP: AxisDirection = AxisDirection {
    axis: Axis::X,
    sign: Sign::Pos,
};
pub const YP: AxisDirection = AxisDirection {
    axis: Axis::Y,
    sign: Sign::Pos,
};
pub const ZP: AxisDirection = AxisDirection {
    axis: Axis::Z,
    sign: Sign::Pos,
};
pub const XN: AxisDirection = AxisDirection {
    axis: Axis::X,
    sign: Sign::Neg,
};
pub const YN: AxisDirection = AxisDirection {
    axis: Axis::Y,
    sign: Sign::Neg,
};
pub const ZN: AxisDirection = AxisDirection {
    axis: Axis::Z,
    sign: Sign::Neg,
};

impl AxisDirection {
    /// Generate direction vector.
    fn direction_vec(&self) -> Vec3 {
        let v = match self.axis {
            Axis::X => Vec3::X,
            Axis::Y => Vec3::Y,
            Axis::Z => Vec3::Z,
        };
        match self.sign {
            Sign::Pos => v,
            Sign::Neg => -v,
        }
    }

    /// Get the four vectors prependicular to self
    /// and along the crossline of the face in order to move vertices.
    /// return order:
    /// 2 1
    ///  o
    /// 3 4
    #[rustfmt::skip]
    fn get_four_vtx(&self) -> [Vec3; 4] {
        let a = vec3( 0.5, 0.5,-0.5);
        let b = vec3(-0.5, 0.5,-0.5);
        let c = vec3(-0.5, 0.5, 0.5);
        let d = vec3( 0.5, 0.5, 0.5);
        let e = vec3( 0.5,-0.5,-0.5);
        let f = vec3(-0.5,-0.5,-0.5);
        let g = vec3(-0.5,-0.5, 0.5);
        let h = vec3( 0.5,-0.5, 0.5);
        match self.sign {
            Sign::Pos => match self.axis {
                Axis::X =>  [a,d,h,e],
                Axis::Y => [a,b,c,d],
                Axis::Z => [d,c,g,h],
            },
            Sign::Neg => match self.axis {
                Axis::X => [c,b,f,g],
                Axis::Y => [h,g,f,e],
                Axis::Z => [b,a,e,f],
            },
        }
    }
}

/// Generate and add a face's mesh into some vertex buffer
pub fn push_face_mesh(
    bukkit: &mut Vec<Vertex>,
    direction: AxisDirection,
    coord: Vec3,
    uv: AtlasCoordinate,
) {
    // Center coord

    let mut c = coord;

    let mut n = direction.get_four_vtx();
    n.iter_mut().for_each(|x| *x = *x + c);

    // b - a d
    // | / / |
    // c e - f
    let aa = uv.aa();
    let bb = uv.bb();
    let mut res = vec![
        Vertex {
            position: n[0].to_array(),
            uv: [bb.x, bb.y],
        },
        Vertex {
            position: n[1].to_array(),
            uv: [aa.x, bb.y],
        },
        Vertex {
            position: n[2].to_array(),
            uv: [aa.x, aa.y],
        },
        Vertex {
            position: n[0].to_array(),
            uv: [bb.x, bb.y],
        },
        Vertex {
            position: n[2].to_array(),
            uv: [aa.x, aa.y],
        },
        Vertex {
            position: n[3].to_array(),
            uv: [bb.x, aa.y],
        },
    ];
    bukkit.append(&mut res);
}

use glam::{Vec2, Vec3};

use super::vertex::Vertex;

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

    /// Get the four vectors along the axis and prependicular to self.
    fn get_four_normals(&self) -> [Vec3; 4] {
        // Fancy trick

        // Flip bits: (-1 0 0) -> (0 1 1) and so on
        let n1 = self
            .direction_vec()
            .to_array()
            .map(|v| if v != 0.0 { 0.0 } else { 1.0 });

        // (0 -1 -1)
        let n2 = n1.map(|x| -x);

        // (0 -1 1)
        let mut n3 = n1;
        for i in &mut n3 {
            if *i == 1.0 {
                *i = -1.0;
                break;
            }
        }

        // (0 1 -1)
        let mut n4 = n2;
        for i in &mut n4 {
            if *i == -1.0 {
                *i = 1.0;
                break;
            }
        }

        [n1.into(), n2.into(), n3.into(), n4.into()]
    }
}

/// Generate and add a face's mesh into some vertex buffer
pub fn add_face_mesh(bukkit: &mut Vec<Vertex>, direction: AxisDirection, coord: Vec3, uv: Vec2) {
    // Center coord
    let mut c = coord;
    let mut vs = [coord; 4];

    let d = direction.direction_vec();

    let mut n = direction.get_four_normals();

    // Move the vertices into target face's center.
    vs = vs.map(|x| x + d);
    n = n.map(|normal| normal * 0.5);

    let vs = n
        .iter()
        .zip(vs.iter())
        .map(|(v, n)| *v + *n)
        .collect::<Vec<Vec3>>();
    let mut vs = vs
        .iter()
        .map(|v| Vertex {
            position: v.to_array(),
            uv: [0.0, 0.0],
        })
        .collect();

    bukkit.append(&mut vs);
}

use glam::Vec3A;

#[derive(Clone, Debug)]
pub struct Triangle {
    pub(crate) points: [usize; 3],
    pub(crate) normals: Option<[usize; 3]>,
    pub(crate) calculated_normal: usize,
    pub(crate) textures: Option<[usize; 3]>,
}

impl Triangle {
    pub(crate) fn normal_from_points(point_a: Vec3A, point_b: Vec3A, point_c: Vec3A) -> Vec3A {
        (point_a - point_b).cross(point_a - point_c).normalize().into()
    }

    pub(crate) fn new() -> Triangle {
        Triangle {
            points: [0; 3],
            normals: Some([3;3]),
            calculated_normal: 0,
            textures: None,
        }
    }
}
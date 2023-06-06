use crate::mesh::solver::solve_indices;
use crate::mesh::{Indices, Normals, Vertices};

use glam::Vec3A;
use glam::Mat4;

pub struct Bone {
    pub(crate) idx: u32,
    pub(crate) mat: Mat4,
    pub(crate) parent: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct Triangle {
    pub(crate) points: [usize; 3],
    pub(crate) normals: Option<[usize; 3]>,
    pub(crate) calculated_normal: usize,
    pub(crate) textures: Option<[usize; 3]>,
}

impl Triangle {
    pub(crate) fn normal_from_points(point_a: Vec3A, point_b: Vec3A, point_c: Vec3A) -> Vec3A {
        (point_a - point_b)
            .cross(point_a - point_c)
            .normalize()
            .into()
    }

    pub(crate) fn new() -> Triangle {
        Triangle {
            points: [0; 3],
            normals: Some([3; 3]),
            calculated_normal: 0,
            textures: None,
        }
    }
}

pub struct Mesh {
    pub(crate) triangles: Vec<Triangle>,
    pub(crate) normals: Normals,
    pub(crate) calculated: Normals,
    pub(crate) vertices: Vertices,
    pub(crate) uvs: Vertices,
    pub(crate) weights: vec<vec<(u32, f32)>>,
    pub(crate) bones: vec<Bone>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            triangles: vec![],
            normals: vec![],
            calculated: vec![],
            vertices: vec![],
            uvs: vec![],
            weights: vec![],
            bones: vec![],
        }
    }

    pub fn as_buffers(&mut self) -> (Indices, Vertices, Vertices, Normals) {
        let (vp, uv, nm, faces) =
            solve_indices(&self.vertices, &self.uvs, &self.normals, &self.triangles);
        (faces.iter().map(|x| *x as u16).collect(), vp, uv, nm)
    }

    pub(crate) fn normal_from_indexes(&self, triangle: &Triangle) -> Vec3A {
        Triangle::normal_from_points(
            self.vertices[triangle.points[0]],
            self.vertices[triangle.points[1]],
            self.vertices[triangle.points[2]],
        )
    }
}

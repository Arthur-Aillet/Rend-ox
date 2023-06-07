use crate::mesh::solver::solve_indices;
use crate::mesh::{Indices, Normals, Vertices};

use glam::Vec3A;
use glam::Mat4;
use crate::mesh::obj_parser::OBJMesh;

#[derive(Clone, Debug, PartialOrd)]
pub struct Bone {
    pub(crate) idx: u32,
    pub(crate) pose: Mat4,
    // pub(crate) rest: Mat4,
    pub(crate) parent: i32,
}

#[derive(Clone, Debug)]
pub(crate) struct Triangle {
    pub(crate) points: [usize; 3],
    pub(crate) normals: Option<[usize; 3]>,
    pub(crate) calculated_normal: usize,
    pub(crate) textures: Option<[usize; 3]>,
    pub(crate) group: Option<u32>,
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
            group: None,
        }
    }
}

pub struct Mesh {
    pub(crate) faces: Indices,
    pub(crate) vertices: Vertices,
    pub(crate) normals: Normals,
    pub(crate) uvs: Vertices,
    pub(crate) weights: Vec<Vec<(u32, f32)>>,
    pub(crate) materials: Vec<Option<String>>,
    pub(crate) groups: Vec<(u32, String)>,
    pub(crate) bones: Vec<Bone>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            faces: vec![],
            vertices: vec![],
            uvs: vec![],
            normals: vec![],
            materials: vec![],
            groups: vec![],
            weights: vec![],
            bones: vec![],
        }
    }

    pub fn buffers(&self) -> (&Indices, &Vertices, &Vertices, &Normals) {
        (&self.faces, &self.vertices, &self.uvs, &self.normals)
    }

    pub fn from_obj( file_name: &str) -> Mesh {
        let mut obj : OBJMesh;
        obj.load_obj(file_name);
        let (faces, vertices, uvs, normals) = obj.as_buffers();
        Mesh {
            faces,
            vertices,
            uvs,
            normals,
            materials: vec![],
            groups: vec![],
            weights: vec![],
            bones: vec![],
        }
    }

    pub(crate) fn normal_from_indexes(&self, triangle: &Triangle) -> Vec3A {
        Triangle::normal_from_points(
            self.vertices[triangle.points[0]],
            self.vertices[triangle.points[1]],
            self.vertices[triangle.points[2]],
        )
    }
}

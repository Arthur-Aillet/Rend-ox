use std::cmp::Ordering;
use crate::mesh::{Indices, Normals, Vertices};
use crate::Vec3;
use crate::error::RendError;

use glam::Mat4;
use crate::mesh::obj_parser::OBJMesh;

#[derive(Clone, Debug)]
pub struct Bone {
    pub(crate) idx: u32,
    pub(crate) pose: Mat4,
    // pub(crate) rest: Mat4,
    pub(crate) parent: i32,
}

impl PartialEq for Bone {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }

}

impl PartialOrd for Bone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.idx.cmp(&other.idx))
    }
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
    pub(crate) fn normal_from_points(point_a: Vec3, point_b: Vec3, point_c: Vec3) -> Vec3 {
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

    pub fn from_obj( file_name: &str) -> Result<Mesh, Box<dyn std::error::Error>> {
        let mut obj = OBJMesh::new();
        if !obj.load_obj(file_name) {
            return Err(Box::new(RendError::new("Failed to parse")));
        }
        let (faces, vertices, uvs, normals) = obj.as_buffers();
        Ok(Mesh {
            faces,
            vertices,
            uvs,
            normals,
            materials: vec![],
            groups: vec![],
            weights: vec![],
            bones: vec![],
        })
    }
}

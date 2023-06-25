//! Generic Mesh class
//!
//! Represents any mesh data loaded from any format
//! Supports:
//!     - custom normal splits
//!     - vertex groups
//!     - material slots
//! planned support for bone animation

use crate::error::RendError;
use crate::mesh::{Indices, Normals, Vertices};
use crate::Vec3;
use std::cmp::Ordering;

use crate::mesh::obj_parser::OBJMesh;
use glam::Mat4;

/// A Bone used for animation
/// Not fully implemented, format subject to change
#[derive(Clone, Debug)]
pub(crate) struct Bone {
    pub(crate) idx: u32,
    pub(crate) _pose: Mat4,
    // pub(crate) rest: Mat4,
    pub(crate) _parent: i32,
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
    pub(crate) _group: Option<u32>,
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
            _group: None,
        }
    }
}

/// A single mesh
///
/// holds geometry data
/// The [`Self::buffers()`] Method gives access to the internal buffers for use in rendering
/// A mesh can be loaded from:
///     - an ascii obj file with [`Self::from_obj()`]
///     - ~~an smd file with [`Self::from_smd()`]~~ Not yet supported
pub struct Mesh {
    pub(crate) path: String,
    pub(crate) faces: Indices,
    pub(crate) vertices: Vertices,
    pub(crate) normals: Normals,
    pub(crate) uvs: Vertices,
    pub(crate) _weights: Vec<Vec<(u32, f32)>>,
    pub(crate) _materials: Vec<Option<String>>,
    pub(crate) _groups: Vec<(u32, String)>,
    pub(crate) _bones: Vec<Bone>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            path: "".into(),
            faces: vec![],
            vertices: vec![],
            uvs: vec![],
            normals: vec![],
            _materials: vec![],
            _groups: vec![],
            _weights: vec![],
            _bones: vec![],
        }
    }

    /// Access the buffers associated with a mesh file, for rendering.
    /// These buffers are prior to any transformation
    pub fn buffers(&self) -> (&Indices, &Vertices, &Vertices, &Normals) {
        (&self.faces, &self.vertices, &self.uvs, &self.normals)
    }

    /// Load an obj from a ASCII .smd file
    /// This function will return an error on an unparsable file
    ///
    /// Not yet implemented, do not use
    #[deprecated(since="0.0.0", note="smd files not yet supported, please use `from_obj` instead")]
    pub fn from_smd(_file_name: &str) -> Result<Mesh, Box<dyn std::error::Error>> {
        Err(Box::new(RendError::new("SMD Not Supported!")))
    }

    /// Load an obj from a ASCII .obj file
    /// This function will return an error on an unparsable file
    pub fn from_obj(file_name: &str) -> Result<Mesh, Box<dyn std::error::Error>> {
        let mut obj = OBJMesh::new();
        if !obj.load_obj(file_name) {
            return Err(Box::new(RendError::new("Failed to parse")));
        }
        let (faces, vertices, uvs, normals) = obj.as_buffers();
        Ok(Mesh {
            path: file_name.into(),
            faces,
            vertices,
            uvs,
            normals,
            _materials: vec![],
            _groups: vec![],
            _weights: vec![],
            _bones: vec![],
        })
    }
}

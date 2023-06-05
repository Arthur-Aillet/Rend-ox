use crate::obj::parser::Triangle;
use crate::obj::{Normals, Vertices};

pub struct Mesh {
    pub(crate) triangles: Vec<Triangle>,
    pub(crate) normals: Normals,
    pub(crate) calculated: Normals,
    pub(crate) vertices: Vertices,
    pub(crate) uvs: Vertices,
}

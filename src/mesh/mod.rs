//! Utilities to load 3D files, currently the only supported format is obj

mod mesh;
mod obj_parser;
mod descriptor;
mod solver;

pub(crate) use crate::Vec3;

pub use descriptor::MeshDescriptor;
pub use mesh::*;

pub(crate) type Vertices = Vec<Vec3>;
pub(crate) type Indices = Vec<u16>;
pub(crate) type Normals = Vec<Vec3>;

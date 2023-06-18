mod mesh;
mod obj_parser;
// mod smd_parser;
mod descriptor;
mod solver;

pub use crate::Vec3;
pub use glam::Mat4;

pub use descriptor::MeshDescriptor;
pub use mesh::*;

pub type Vertices = Vec<Vec3>;
pub type Indices = Vec<u16>;
pub type Normals = Vec<Vec3>;

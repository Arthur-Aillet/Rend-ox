mod mesh;
mod obj_parser;
// mod smd_parser;
mod solver;
mod descriptor;

pub use crate::Vec3;
pub use glam::Mat4;

pub use mesh::*;
pub use descriptor::MeshDescriptor;

pub type Vertices = Vec<Vec3>;
pub type Indices = Vec<u16>;
pub type Normals = Vec<Vec3>;

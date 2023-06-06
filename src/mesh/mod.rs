mod mesh;
mod obj_parser;
mod smd_parser;
mod solver;

use glam::Vec3A;
use glam::Mat4;

pub use mesh::*;

pub type Vertices = Vec<Vec3A>;
pub type Indices = Vec<u16>;
pub type Normals = Vec<Vec3A>;

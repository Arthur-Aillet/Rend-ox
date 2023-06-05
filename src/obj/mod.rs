mod mesh;
mod parser;
mod solver;

use glam::Vec3A;

pub use mesh::*;

pub type Vertices = Vec<Vec3A>;
pub type Indices = Vec<u16>;
pub type Normals = Vec<Vec3A>;

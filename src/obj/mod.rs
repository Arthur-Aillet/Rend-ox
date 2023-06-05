mod mesh;
mod parser;
mod solver;

use glam::Vec3A;
pub use mesh::*;
use parser::Triangle;

use crate::obj::solver::solve_indices;

pub type Vertices = Vec<Vec3A>;
pub type Indices = Vec<u16>;
pub type Normals = Vec<Vec3A>;

impl Mesh {
    pub fn as_buffers(&mut self) -> (Indices, Vertices, Vertices, Normals) {
        let (vp, uv, nm, faces) =
            solve_indices(&self.vertices, &self.uvs, &self.normals, &self.triangles);
        (faces.iter().map(|x| *x as u16).collect(), vp, uv, nm)
    }

    fn normal_from_indexes(&self, triangle: &Triangle) -> Vec3A {
        Triangle::normal_from_points(
            self.vertices[triangle.points[0]],
            self.vertices[triangle.points[1]],
            self.vertices[triangle.points[2]],
        )
    }

    pub fn new() -> Mesh {
        Mesh {
            triangles: vec![],
            normals: vec![],
            calculated: vec![],
            vertices: vec![],
            uvs: vec![],
        }
    }
}

use glam::Vec3A;

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

use super::{Indices, Triangle, Normals, Vertices, Bone};
use super::Mesh;
use super::solver::solve_indices;

pub struct OBJMesh {
    pub(crate) triangles: Vec<Triangle>,
    pub(crate) normals: Normals,
    pub(crate) calculated: Normals,
    pub(crate) vertices: Vertices,
    pub(crate) uvs: Vertices,
    pub(crate) materials: Vec<Option<String>>,
}

impl OBJMesh {
    pub fn as_buffers(&self) -> (Indices, Vertices, Vertices, Normals) {
        let (vp, uv, nm, faces) =
            solve_indices(&self.vertices, &self.uvs, &self.normals, &self.triangles);
        (faces.iter().map(|x| *x as u16).collect(), vp, uv, nm)
    }

    pub fn load_obj(&mut self, file_name: &str) -> bool {
        let file = OpenOptions::new().read(true).open(file_name);

        if let Ok(obj) = file {
            for option_line in BufReader::new(obj).lines() {
                match option_line {
                    Err(why) => panic!("{:?}", why),
                    Ok(line) => match line {
                        s if s.chars().all(|x| x.is_ascii_whitespace()) => {
                            continue;
                        }
                        s if s.starts_with('#') => {
                            continue;
                        }
                        s if s.starts_with("o ") => {
                            continue;
                        }
                        s if s.starts_with("s ") => {
                            continue;
                        }
                        s if s.starts_with("usemtl ") => {
                            continue;
                        }
                        s if s.starts_with("mtllib ") => {
                            continue;
                        }
                        s if s.starts_with("v ") => {
                            if let Some(vertex) = self.parse_vertex(s) {
                                self.vertices.push(vertex);
                            } else {
                                println!("Invalid vertexes in \"{}\" !", file_name);
                                return false;
                            }
                        }
                        s if s.starts_with("vt ") => {
                            if let Some(vertex) = self.parse_uvw(s) {
                                self.uvs.push(vertex);
                            } else {
                                println!("Invalid uv in \"{}\" !", file_name);
                                return false;
                            }
                        }
                        s if s.starts_with("vn ") => {
                            if let Some(normal) = self.parse_normal(s) {
                                self.normals.push(normal);
                            } else {
                                println!("Invalid normal in \"{}\" !", file_name);
                                return false;
                            }
                        }
                        s if s.starts_with("f ") => {
                            if !self.parse_face(s) {
                                println!("Invalid face in \"{}\" !", file_name);
                                return false;
                            }
                        }
                        other => {
                            println!("Invalid \"{file_name}\" mesh file!, \"{other}\" string not supported");
                            return false;
                        }
                    },
                }
            }
        } else {
            println!("Cant open \"{}\" mesh file!", file_name);
        }
        true
    }

    fn parse_face_point(&self, point: &str) -> Option<(usize, Option<usize>, Option<usize>)> {
        let iter = &mut point.split('/');
        let position: usize;
        let normal: Option<usize>;
        let texture: Option<usize>;

        if let Some(str) = iter.next() {
            match str.parse::<usize>() {
                Ok(x) => {
                    if self.vertices.len() >= x && x > 0 {
                        position = x - 1
                    } else {
                        return None;
                    }
                }
                Err(_) => return None,
            };
        } else {
            return None;
        }

        if let Some(str) = iter.next() {
            if str.is_empty() {
                return Some((position, None, None));
            } else {
                match str.parse::<usize>() {
                    Ok(x) => {
                        if self.uvs.len() >= x && x > 0 {
                            texture = Some(x - 1);
                        } else {
                            return None;
                        }
                    }
                    Err(_) => return None,
                }
            }
        } else {
            return None;
        }

        if let Some(str) = iter.next() {
            if str.is_empty() {
                return Some((position, None, None));
            } else {
                match str.parse::<usize>() {
                    Ok(x) => {
                        if self.normals.len() >= x && x > 0 {
                            normal = Some(x - 1);
                        } else {
                            return None;
                        }
                    }
                    Err(_) => return None,
                }
            }
        } else {
            return None;
        }
        Some((position, texture, normal))
    }

    fn parse_second_face(&mut self, fst_triangle: &Triangle, last_point: &str) -> Option<Triangle> {
        let mut points: [usize; 3] = [fst_triangle.points[1], fst_triangle.points[2], 0];
        let mut textures = match fst_triangle.textures {
            Some(tex) => Some([tex[1], tex[2], 0]),
            None => None,
        };
        let mut normals = match fst_triangle.textures {
            Some(nor) => Some([nor[1], nor[2], 0]),
            None => None,
        };

        if let Some(indexes) = self.parse_face_point(last_point) {
            points[2] = indexes.0;

            match indexes.1 {
                Some(index) => {
                    if let Some(ref mut point_texture) = textures {
                        point_texture[2] = index;
                    }
                }
                None => textures = None,
            }
            match indexes.2 {
                Some(index) => {
                    if let Some(ref mut point_normal) = normals {
                        point_normal[2] = index;
                    }
                }
                None => normals = None,
            }
        } else {
            return None;
        }
        let calculated_normal = self.calculated.len();
        let snd_triangle = Triangle {
            points,
            normals,
            calculated_normal,
            textures,
            group : None,
        };
        self.calculated
            .push(self.normal_from_indexes(&snd_triangle));
        Some(snd_triangle)
    }

    fn parse_face(&mut self, line: String) -> bool {
        let points: Vec<&str> = line.split_ascii_whitespace().skip(1).collect();
        let len = points.len();
        if len < 3 || len > 4 {
            return false;
        }
        let mut fst_triangle = Triangle::new();
        fst_triangle.normals = Some([0; 3]);
        fst_triangle.textures = Some([0; 3]);
        for i in 0..3 {
            if let Some(point) = self.parse_face_point(points[i]) {
                fst_triangle.points[i] = point.0;

                if let Some(ref mut texture) = fst_triangle.textures {
                    match point.1 {
                        Some(valid_point) => texture[i] = valid_point,
                        None => fst_triangle.textures = None,
                    }
                }
                if let Some(ref mut normal) = fst_triangle.normals {
                    match point.2 {
                        Some(valid_point) => normal[i] = valid_point,
                        None => fst_triangle.normals = None,
                    }
                }
            } else {
                return false;
            }
        }
        fst_triangle.calculated_normal = self.calculated.len();

        self.calculated
            .push(self.normal_from_indexes(&fst_triangle));
        if len == 4 {
            if let Some(snd_face) = self.parse_second_face(&fst_triangle, points[3]) {
                self.triangles.push(snd_face);
            } else {
                return false;
            }
        }
        self.triangles.push(fst_triangle);
        true
    }

    fn parse_vertex(&mut self, line: String) -> Option<Vec3A> {
        let mut new_vertex: Vec3A = Vec3A::ZERO;
        let mut iter = line.split_ascii_whitespace().filter(|&x| !x.is_empty());

        iter.next();
        for i in 0..3 {
            if let Some(point) = iter.next() {
                if let Ok(point) = point.parse::<f32>() {
                    new_vertex[i] = point as f32;
                } else {
                    return None;
                };
            } else {
                return None;
            }
        }
        Some(new_vertex)
    }

    fn parse_uvw(&mut self, line: String) -> Option<Vec3A> {
        let mut new_vertex: Vec3A = Vec3A::ZERO;
        let mut iter = line.split_ascii_whitespace().filter(|&x| !x.is_empty());

        iter.next();
        for i in 0..3 {
            if let Some(point) = iter.next() {
                if let Ok(point) = point.parse::<f32>() {
                    new_vertex[i] = point as f32;
                } else {
                    return None;
                };
            }
        }
        Some(new_vertex)
    }

    fn parse_normal(&mut self, line: String) -> Option<Vec3A> {
        let new_normal_asv = self.parse_vertex(line);
        if let Some(normal_vertex) = new_normal_asv {
            Some(Vec3A::from(normal_vertex.normalize()))
        } else {
            None
        }
    }
}

use glam::Vec3A;
use glam::Mat4;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::collections::hash_map::HashMap;
use std::collections::HashSet;

use super::Triangle;
use super::Mesh;
use super::Bone;

struct Rig {
    pub(crate) groups: HashMap<u32, String>,
    pub(crate) bones: HashMap<u32, Bone>,
}

impl Rig {
    pub fn new() -> Rig {
        Rig {
            groups: HashMap::new(),
            bones: HashMap::new(),
        }
    }
}

impl Mesh {
    pub fn load_smd(&mut self, file_name: &str) -> bool {
        let file = OpenOptions::new().read(true).open(file_name);

        if let Ok(obj) = file {
            self.smd_load_nodes(&obj);
            self.smd_load_skeleton(&obj);
            self.smd_load_triangles(&obj);
        } else {
            println!("Cant open \"{}\" mesh file!", file_name);
        }
        true
    }

    fn smd_parse_node(&self, &mut rig_data : &mut Rig, line: String) -> bool {
        let idx : u32;
        let name : str;
        let parent : i32;

        let mut iter = line.split_ascii_whitespace().filter(|&x| !x.is_empty());

        if let Some(word) = iter.next() {
            if let Ok(parse) = word.parse::<u32>() {
                idx = parse as u32;
            } else {
                return false;
            };
        } else {
            return false;
        }

        if let Some(word) = iter.next() {
            name = word.into();
        } else {
            return false;
        }

        if let Some(word) = iter.next() {
            if let Ok(parse) = word.parse::<i32>() {
                if parent < idx as i32 {
                    parent = parse as i32;
                } else {
                    return false;
                }
            } else {
                return false;
            };
        } else {
            return false;
        }

        rig_data.groups.push(idx, name);
        rig_data.bones.push(idx, Bone {
            idx,
            pose : Mat4::IDENTITY,
            parent,
        });

        false
    }

    fn smd_load_nodes(&mut self, &file: &File) -> bool {
        let mut rig_data = Rig::new();
        for option_line in BufReader::new(file).lines() {
            match option_line {
                Err(why) => panic!("{:?}", why),
                Ok(line) => match line {
                    s if s.chars().all(|x| x.is_ascii_whitespace()) => {
                        continue;
                    }
                    s if s.starts_with("end") => {
                        break;
                    }
                    s => {
                        if !self.smd_parse_node(&mut rig_data, s) {
                            return false;
                        }
                    },
                }
            }
        }

        self.groups = rig_data.groups.into();
        self.bones = rig_data.bones.into();
        self.groups.sort();
        self.bones.sort();

        true
    }

    fn smd_load_skeleton(&mut self, &file: &File) -> bool {
        true
    }

    fn smd_parse_corner(&self, &mut tri: &mut Triangle, state : u32, line : String) -> bool {
        false
    }

    fn smd_load_triangles(&mut self, &file: &File) -> bool {
        let mut materials = HashMap!();
        let mut mat_count = 0;
        let mut curr_material : Option<u32> = None;
        let mut state = 0;
        let mut tri = Triangle::new();
        for option_line in BufReader::new(file).lines() {
            match option_line {
                Err(why) => panic!("{:?}", why),
                Ok(line) => match line {
                    s if s.chars().all(|x| x.is_ascii_whitespace()) => {
                        continue;
                    }
                    s if s.starts_with("end") => {
                        break;
                    }
                    s => {
                        if state == 0 {
                            curr_material = materials.entry(&s).or_insert(materials.len);
                        } else if state > 3 {
                            return false;
                        } else {
                            if !self.smd_parse_corner(&mut tri, state, s) {
                                return false;
                            }
                        }
                        state += 1;
                    },
                }
            }
        }
        true
    }
}

/*

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
 // /*  */
 */
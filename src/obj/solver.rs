use crate::obj::parser::Triangle;
use glam::Vec3A;

#[derive(Clone)]
struct VertBinding {
    pub idx: usize,
    pub uv: usize,
    pub nm: usize,
}

fn binding_find(
    binding: &mut Vec<Vec<VertBinding>>,
    idx: usize,
    vp: usize,
    uv: usize,
    nm: usize,
) -> (bool, usize) {
    for corner in &binding[vp] {
        if corner.uv == uv && corner.nm == nm {
            return (true, corner.idx);
        }
    }

    binding[vp].push(VertBinding { idx, uv, nm });

    (false, idx)
}

pub(crate) fn solve_indices(
    pos: &Vec<Vec3A>,
    uvs: &Vec<Vec3A>,
    normals: &Vec<Vec3A>,
    faces: &Vec<Triangle>,
) -> (Vec<Vec3A>, Vec<Vec3A>, Vec<Vec3A>, Vec<usize>) {
    let mut out_vp: Vec<Vec3A> = Vec::new();
    let mut out_nm: Vec<Vec3A> = Vec::new();
    let mut out_uv: Vec<Vec3A> = Vec::new();
    let mut binding: Vec<Vec<VertBinding>> = vec![Vec::new(); pos.len()];

    let mut out_faces: Vec<usize> = Vec::new();

    let mut counter = 0;

    for face in faces {
        for i in 0..3 {
            let (found, idx) = binding_find(
                &mut binding,
                counter,
                face.points[i],
                face.textures.unwrap_or([0; 3])[i],
                face.normals.unwrap_or([0; 3])[i],
            );
            if !found {
                counter += 1;
                out_vp.push(pos[face.points[i]]);
                out_uv.push(uvs[face.textures.unwrap_or([0; 3])[i]]);
                out_nm.push(normals[face.normals.unwrap_or([0; 3])[i]]);
            }
            out_faces.push(idx);
        }
    }

    (out_vp, out_uv, out_nm, out_faces)
}

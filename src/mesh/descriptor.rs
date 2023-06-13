//! Mesh Descriptor
//!
//! Game-side draw calls will be done on these mesh descriptors
//! to avoid having to deal with messy meshes (including expensive copies and such)
//!

use crate::Mat4;
use crate::app::App;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshDescriptor {
    pub name: String,
    pub(crate) idx: usize,
    pub(crate) shader: usize,
}

impl MeshDescriptor {
    pub fn new(idx: usize, path: &str, shader: usize) -> MeshDescriptor {
        MeshDescriptor {
            name: path.into(),
            idx,
            shader,
        }
    }
}

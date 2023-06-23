//! Mesh Descriptor
//!
//! Game-side draw calls will be done on these mesh descriptors
//! to avoid having to deal with messy meshes (including expensive copies and such)
//!

use crate::graphics::{MaterialSlot, MeshSlot};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshDescriptor {
    pub name: String,
    pub(crate) idx: MeshSlot,
    pub(crate) material: MaterialSlot,
}

impl MeshDescriptor {
    pub fn new(idx: MeshSlot, path: &str, material: MaterialSlot) -> MeshDescriptor {
        MeshDescriptor {
            name: path.into(),
            idx,
            material,
        }
    }
}

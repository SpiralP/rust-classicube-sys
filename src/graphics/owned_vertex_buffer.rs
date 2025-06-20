use crate::{
    bindings::{GfxResourceID, Gfx_CreateDynamicVb, Gfx_DeleteVb, VertexFormat},
    std_types::c_int,
};

pub struct OwnedGfxVertexBuffer {
    pub resource_id: GfxResourceID,
}

impl OwnedGfxVertexBuffer {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(fmt: VertexFormat, max_vertices: c_int) -> Self {
        let resource_id = unsafe { Gfx_CreateDynamicVb(fmt, max_vertices) };

        assert!(resource_id as usize != 0);

        Self { resource_id }
    }
}

impl Drop for OwnedGfxVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            Gfx_DeleteVb(&mut self.resource_id);
        }
    }
}

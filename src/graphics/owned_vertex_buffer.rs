use crate::{bindings::*, std_types::c_int};

pub struct OwnedGfxVertexBuffer {
    pub resource_id: GfxResourceID,
}

impl OwnedGfxVertexBuffer {
    /// Returns `None` if the GPU rejects the buffer — typically because the
    /// graphics context is currently lost (mid-device-reset on Windows D3D9).
    #[must_use]
    pub fn new(fmt: VertexFormat, max_vertices: c_int) -> Option<Self> {
        let resource_id = unsafe { Gfx_CreateDynamicVb(fmt, max_vertices) };

        if resource_id as usize == 0 {
            return None;
        }

        Some(Self { resource_id })
    }
}

impl Drop for OwnedGfxVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            Gfx_DeleteVb(&raw mut self.resource_id);
        }
    }
}

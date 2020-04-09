use crate::bindings::*;

pub struct OwnedGfxVertexBuffer {
    pub resource_id: GfxResourceID,
}

impl OwnedGfxVertexBuffer {
    pub fn create(fmt: VertexFormat, max_vertices: ::std::os::raw::c_int) -> Self {
        let resource_id = unsafe { Gfx_CreateDynamicVb(fmt, max_vertices) };
        println!("Gfx_CreateVertexBuffer {:#?}", resource_id);

        assert!(resource_id as usize != 0);

        Self { resource_id }
    }
}

impl Drop for OwnedGfxVertexBuffer {
    fn drop(&mut self) {
        println!("Gfx_DeleteVb {:#?}", self.resource_id);
        unsafe {
            Gfx_DeleteVb(&mut self.resource_id);
        }
    }
}

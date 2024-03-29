use crate::bindings::*;

pub struct OwnedGfxTexture {
    pub resource_id: GfxResourceID,
}

impl OwnedGfxTexture {
    /// # Panics
    ///
    /// Will panic if `bmp` doesn't have a power of two dimensions.
    pub fn new(bmp: &mut Bitmap, managed_pool: bool, mipmaps: bool) -> Self {
        let resource_id =
            unsafe { Gfx_CreateTexture(bmp, u8::from(managed_pool), u8::from(mipmaps)) };

        assert!(resource_id as usize != 0);

        Self { resource_id }
    }
}

impl Drop for OwnedGfxTexture {
    fn drop(&mut self) {
        unsafe {
            Gfx_DeleteTexture(&mut self.resource_id);
        }
    }
}

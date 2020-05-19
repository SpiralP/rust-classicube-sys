use crate::bindings::*;

pub struct OwnedGfxTexture {
    pub resource_id: GfxResourceID,
}

impl OwnedGfxTexture {
    pub fn create(bmp: &mut Bitmap, managed_pool: bool, mipmaps: bool) -> Self {
        let resource_id = unsafe {
            Gfx_CreateTexture(
                bmp,
                if managed_pool { 1 } else { 0 },
                if mipmaps { 1 } else { 0 },
            )
        };

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

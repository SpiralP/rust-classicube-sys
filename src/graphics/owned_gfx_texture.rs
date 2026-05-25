use crate::bindings::*;

pub struct OwnedGfxTexture {
    pub resource_id: GfxResourceID,
}

impl OwnedGfxTexture {
    /// Returns `None` if the GPU rejects the bitmap — e.g. the graphics
    /// context is currently lost (mid-device-reset on Windows D3D9) or the
    /// power-of-two-rounded dimensions exceed the backend's texture limits.
    pub fn new(bmp: &mut Bitmap, managed_pool: bool, mipmaps: bool) -> Option<Self> {
        let resource_id =
            unsafe { Gfx_CreateTexture(bmp, u8::from(managed_pool), u8::from(mipmaps)) };

        if resource_id as usize == 0 {
            return None;
        }

        Some(Self { resource_id })
    }
}

impl Drop for OwnedGfxTexture {
    fn drop(&mut self) {
        unsafe {
            Gfx_DeleteTexture(&mut self.resource_id);
        }
    }
}

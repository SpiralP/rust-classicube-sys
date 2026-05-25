use core::borrow::Borrow;

use crate::{
    OwnedGfxTexture,
    bindings::{Bitmap, Texture, TextureRec, cc_uint16},
    std_types::c_short,
};

pub struct OwnedTexture {
    texture: Texture,

    #[allow(dead_code)]
    gfx_texture: OwnedGfxTexture,
}

impl OwnedTexture {
    /// Returns `None` if `OwnedGfxTexture::new` rejects the bitmap — see its
    /// docs for the conditions under which the GPU can refuse the upload.
    pub fn new(
        bmp: &mut Bitmap,
        coords: (c_short, c_short),
        size: (cc_uint16, cc_uint16),
        uv: TextureRec,
    ) -> Option<Self> {
        let gfx_texture = OwnedGfxTexture::new(bmp, true, false)?;
        let texture = Texture {
            ID: gfx_texture.resource_id,
            x: coords.0,
            y: coords.1,
            width: size.0,
            height: size.1,
            uv,
        };

        Some(Self {
            texture,
            gfx_texture,
        })
    }

    #[must_use]
    pub fn as_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn as_texture_mut(&mut self) -> &mut Texture {
        &mut self.texture
    }

    /// # Safety
    ///
    /// The `OwnedTexture` needs to live longer than the `Texture` return here.
    #[must_use]
    pub unsafe fn get_texture(&self) -> Texture {
        Texture { ..self.texture }
    }
}

impl Borrow<Texture> for OwnedTexture {
    fn borrow(&self) -> &Texture {
        self.as_texture()
    }
}

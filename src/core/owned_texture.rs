use crate::{
    bindings::{cc_uint16, Bitmap, Texture, TextureRec},
    OwnedGfxTexture,
};
use std::os::raw::c_short;

pub struct OwnedTexture {
    texture: Texture,

    #[allow(dead_code)]
    gfx_texture: OwnedGfxTexture,
}

impl OwnedTexture {
    pub fn new(
        bmp: &mut Bitmap,
        coords: (c_short, c_short),
        size: (cc_uint16, cc_uint16),
        uv: TextureRec,
    ) -> Self {
        let gfx_texture = OwnedGfxTexture::new(bmp, true, false);
        let texture = Texture {
            ID: gfx_texture.resource_id,
            X: coords.0,
            Y: coords.1,
            Width: size.0,
            Height: size.1,
            uv,
        };

        Self {
            gfx_texture,
            texture,
        }
    }

    pub fn as_texture(&self) -> &Texture {
        &self.texture
    }

    /// # Safety
    ///
    /// The `OwnedTexture` needs to live longer than the `Texture` return here.
    pub unsafe fn get_texture(&self) -> Texture {
        Texture { ..self.texture }
    }
}

impl std::borrow::Borrow<Texture> for OwnedTexture {
    fn borrow(&self) -> &Texture {
        self.as_texture()
    }
}

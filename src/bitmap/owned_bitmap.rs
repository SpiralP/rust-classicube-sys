use crate::{
    bindings::{Bitmap, BitmapCol},
    Math_NextPowOf2,
};
use std::{os::raw::c_int, pin::Pin};

pub struct OwnedBitmap {
    bitmap: Bitmap,

    #[allow(dead_code)]
    #[allow(clippy::box_collection)]
    pixels: Pin<Box<Vec<BitmapCol>>>,
}

impl OwnedBitmap {
    pub fn new(width: c_int, height: c_int, color: BitmapCol) -> Self {
        let mut pixels = Box::pin(vec![color; width as usize * height as usize]);
        let scan0 = unsafe { pixels.as_mut().get_unchecked_mut().as_mut_ptr() };

        Self {
            pixels,
            bitmap: Bitmap {
                width,
                height,
                scan0,
            },
        }
    }

    pub fn new_pow_of_2(width: c_int, height: c_int, color: BitmapCol) -> OwnedBitmap {
        let width = Math_NextPowOf2(width);
        let height = Math_NextPowOf2(height);

        Self::new(width, height, color)
    }

    pub fn new_pow_of_2_cleared(width: c_int, height: c_int) -> OwnedBitmap {
        Self::new_pow_of_2(width, height, 0x0000_0000)
    }

    pub fn as_bitmap(&self) -> &Bitmap {
        &self.bitmap
    }

    pub fn as_bitmap_mut(&mut self) -> &mut Bitmap {
        &mut self.bitmap
    }

    /// # Safety
    ///
    /// The `OwnedBitmap` needs to live longer than the `Bitmap` return here.
    pub unsafe fn get_bitmap(&self) -> Bitmap {
        Bitmap { ..self.bitmap }
    }
}

impl std::borrow::Borrow<Bitmap> for OwnedBitmap {
    fn borrow(&self) -> &Bitmap {
        self.as_bitmap()
    }
}

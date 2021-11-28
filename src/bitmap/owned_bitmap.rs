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
    pub fn new(width: usize, height: usize, color: BitmapCol) -> Self {
        let mut pixels = Box::pin(vec![color; width as usize * height as usize]);
        let scan0 = unsafe { pixels.as_mut().get_unchecked_mut().as_mut_ptr() };

        Self {
            pixels,
            bitmap: Bitmap {
                width: width as _,
                height: height as _,
                scan0,
            },
        }
    }

    pub fn as_bitmap(&self) -> &Bitmap {
        &self.bitmap
    }

    /// # Safety
    ///
    /// The `OwnedBitmap` needs to live longer than the `Bitmap` return here.
    pub unsafe fn get_bitmap(&self) -> Bitmap {
        Bitmap {
            scan0: self.bitmap.scan0,
            width: self.bitmap.width,
            height: self.bitmap.height,
        }
    }
}

pub fn Bitmap_AllocateClearedPow2(width: c_int, height: c_int) -> OwnedBitmap {
    let width = Math_NextPowOf2(width);
    let height = Math_NextPowOf2(height);

    OwnedBitmap::new(width as _, height as _, 0x00FF_FFFF)
}

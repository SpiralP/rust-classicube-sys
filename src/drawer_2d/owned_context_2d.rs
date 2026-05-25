use core::{borrow::Borrow, ptr};

use crate::{
    Math_NextPowOf2,
    bindings::{Bitmap, BitmapCol, Context2D},
    std_types::{Vec, c_int, vec},
};

pub struct OwnedContext2D {
    context_2d: Context2D,

    #[allow(dead_code)]
    pixels: Vec<BitmapCol>,
}

impl OwnedContext2D {
    /// # Panics
    ///
    /// Panics if `width` or `height` is negative.
    #[must_use]
    pub fn new(width: c_int, height: c_int, color: BitmapCol) -> Self {
        let w = usize::try_from(width).expect("context width must be non-negative");
        let h = usize::try_from(height).expect("context height must be non-negative");
        let mut pixels = vec![color; w * h];
        let scan0 = pixels.as_mut_ptr();

        Self {
            context_2d: Context2D {
                bmp: Bitmap {
                    scan0,
                    width,
                    height,
                },
                width,
                height,
                meta: ptr::null_mut(),
            },
            pixels,
        }
    }

    #[must_use]
    pub fn new_cleared(width: c_int, height: c_int) -> Self {
        Self::new(width, height, 0x0000_0000)
    }

    #[must_use]
    pub fn new_pow_of_2(width: c_int, height: c_int, color: BitmapCol) -> OwnedContext2D {
        let width = Math_NextPowOf2(width);
        let height = Math_NextPowOf2(height);

        Self::new(width, height, color)
    }

    #[must_use]
    pub fn new_pow_of_2_cleared(width: c_int, height: c_int) -> OwnedContext2D {
        Self::new_pow_of_2(width, height, 0x0000_0000)
    }

    #[must_use]
    pub fn as_context_2d(&self) -> &Context2D {
        &self.context_2d
    }

    pub fn as_context_2d_mut(&mut self) -> &mut Context2D {
        &mut self.context_2d
    }

    #[must_use]
    pub fn as_bitmap(&self) -> &Bitmap {
        &self.context_2d.bmp
    }

    pub fn as_bitmap_mut(&mut self) -> &mut Bitmap {
        &mut self.context_2d.bmp
    }
}

impl Borrow<Bitmap> for OwnedContext2D {
    fn borrow(&self) -> &Bitmap {
        self.as_bitmap()
    }
}

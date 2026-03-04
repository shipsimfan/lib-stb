use crate::truetype::stbtt_fontinfo;
use std::ffi::{c_float, c_int, c_uchar};

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// Allocates a large-enough single-channel 8bpp bitmap and renders the specified
    /// character/glyph at the specified scale into it, with antialiasing. 0 is no coverage
    /// (transparent), 255 is fully covered (opaque). `*width` & `*height` are filled out with the
    /// width & height of the bitmap, which is stored left-to-right, top-to-bottom.
    ///
    /// `xoff`/`yoff` are the offset it pixel space from the glyph origin to the top-left of the
    /// bitmap
    pub fn stbtt_GetCodepointBitmap(
        info: *const stbtt_fontinfo,
        scale_x: c_float,
        scale_y: c_float,
        codepoint: c_int,
        width: *mut c_int,
        height: *mut c_int,
        xoff: *mut c_int,
        yoff: *mut c_int,
    ) -> *mut c_uchar;
}

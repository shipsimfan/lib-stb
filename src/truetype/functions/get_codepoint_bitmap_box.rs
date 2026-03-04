use crate::truetype::stbtt_fontinfo;
use std::ffi::{c_float, c_int};

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// Get the bbox of the bitmap centered around the glyph origin; so the bitmap width is
    /// `ix1 - ix0`, height is `iy1 - iy0`, and location to place the bitmap top left is
    /// `(left_side_bearing * scale, iy0)`. (Note that the bitmap uses y-increases-down, but the
    /// shape uses y-increases-up, so `CodepointBitmapBox` and `CodepointBox` are inverted.)
    pub fn stbtt_GetCodepointBitmapBox(
        info: *const stbtt_fontinfo,
        codepoint: c_int,
        scale_x: c_float,
        scale_y: c_float,
        ix0: *mut c_int,
        iy0: *mut c_int,
        ix1: *mut c_int,
        iy1: *mut c_int,
    );
}

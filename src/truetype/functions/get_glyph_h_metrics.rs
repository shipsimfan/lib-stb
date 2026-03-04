use crate::truetype::stbtt_fontinfo;
use std::ffi::c_int;

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// Get the horizontal metrics for a single glyph
    pub fn stbtt_GetGlyphHMetrics(
        info: *const stbtt_fontinfo,
        glyph_index: c_int,
        advance_width: *mut c_int,
        left_side_bearing: *mut c_int,
    );
}

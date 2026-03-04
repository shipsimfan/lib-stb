use crate::truetype::stbtt_fontinfo;
use std::ffi::c_int;

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// Get the kerning advance between two glyphs
    pub fn stbtt_GetGlyphKernAdvance(
        info: *const stbtt_fontinfo,
        glyph1: c_int,
        glyph2: c_int,
    ) -> c_int;
}

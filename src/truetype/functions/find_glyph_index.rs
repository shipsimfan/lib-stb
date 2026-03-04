use crate::truetype::stbtt_fontinfo;
use std::ffi::c_int;

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// If you're going to perform multiple operations on the same character and you want a
    /// speed-up, call this function with the character you're going to process, then use
    /// glyph-based functions instead of the codepoint-based functions.
    ///
    /// Returns 0 if the character codepoint is not defined in the font.
    pub fn stbtt_FindGlyphIndex(info: *const stbtt_fontinfo, unicode_codepoint: c_int) -> c_int;
}

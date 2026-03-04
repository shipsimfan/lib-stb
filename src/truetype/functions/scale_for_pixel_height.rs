use crate::truetype::stbtt_fontinfo;
use std::ffi::c_float;

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// Computes a scale factor to produce a font whose "height" is 'pixels' tall.
    ///
    /// Height is measured as the distance from the highest ascender to the lowest descender; in
    /// other words, it's equivalent to calling [`stbtt_GetFontVMetrics`] and computing:
    /// `scale = pixels / (ascent - descent)`, so if you prefer to measure height by the ascent
    /// only, use a similar calculation.
    pub fn stbtt_ScaleForPixelHeight(info: *const stbtt_fontinfo, pixels: c_float) -> c_float;
}

use crate::truetype::stbtt_fontinfo;
use std::ffi::c_int;

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// `ascent` is the coordinate above the baseline the font extends; `descent` is the coordinate
    /// below the baseline the font extends (i.e. it is typically negative) `line_gap` is the
    /// spacing between one row's descent and the next row's ascent... so you should advance the
    /// vertical position by `*ascent - *descent + *line_gap` these are expressed in unscaled
    /// coordinates, so you must multiply by the scale factor for a given size
    pub fn stbtt_GetFontVMetrics(
        info: *const stbtt_fontinfo,
        ascent: *mut c_int,
        descent: *mut c_int,
        line_gap: *mut c_int,
    );
}

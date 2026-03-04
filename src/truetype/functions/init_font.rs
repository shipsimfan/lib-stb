use crate::truetype::stbtt_fontinfo;
use std::ffi::{c_int, c_uchar};

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// Given an offset into the file that defines a font, this function builds
    /// the necessary cached info for the rest of the system. You must allocate
    /// the stbtt_fontinfo yourself, and stbtt_InitFont will fill it out. You don't
    /// need to do anything special to free it, because the contents are pure
    /// value data with no additional data structures. Returns 0 on failure.
    pub fn stbtt_InitFont(info: *mut stbtt_fontinfo, data: *const c_uchar, offset: c_int) -> c_int;
}

use std::ffi::c_void;

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// Frees the bitmap
    pub fn stbtt_FreeBitmap(bitmap: *mut u8, userdata: *mut c_void);
}

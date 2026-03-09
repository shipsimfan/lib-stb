use crate::image::stbi_uc;
use std::ffi::{c_char, c_int};

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    #[allow(missing_docs)]
    pub fn stbi_load(
        filename: *const c_char,
        x: *mut c_int,
        y: *mut c_int,
        channels_in_file: *mut c_int,
        desired_channels: c_int,
    ) -> *mut stbi_uc;
}

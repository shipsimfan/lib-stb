use std::ffi::c_int;

/// only used for `desired_channels`
#[allow(non_upper_case_globals)]
pub const STBI_default: c_int = 0;

#[allow(non_upper_case_globals, missing_docs)]
pub const STBI_grey: c_int = 1;

#[allow(non_upper_case_globals, missing_docs)]
pub const STBI_grey_alpha: c_int = 2;

#[allow(non_upper_case_globals, missing_docs)]
pub const STBI_rgb: c_int = 3;

#[allow(non_upper_case_globals, missing_docs)]
pub const STBI_rgb_alpha: c_int = 4;

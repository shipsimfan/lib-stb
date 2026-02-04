use std::{
    ffi::{c_int, c_uchar},
    ptr::null_mut,
};

/// private structure
#[repr(C)]
#[derive(Debug, Clone)]
pub struct stbtt__buf {
    #[allow(missing_docs)]
    pub data: *mut c_uchar,

    #[allow(missing_docs)]
    pub cursor: c_int,

    #[allow(missing_docs)]
    pub size: c_int,
}

impl Default for stbtt__buf {
    fn default() -> Self {
        stbtt__buf {
            data: null_mut(),
            cursor: 0,
            size: 0,
        }
    }
}

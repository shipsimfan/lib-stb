use std::ffi::c_char;

#[link(name = "stb", kind = "static")]
unsafe extern "C" {
    /// get a VERY brief reason for failure
    ///
    /// on most compilers (and ALL modern mainstream compilers) this is threadsafe
    pub fn stbi_failure_reason() -> *mut c_char;
}
